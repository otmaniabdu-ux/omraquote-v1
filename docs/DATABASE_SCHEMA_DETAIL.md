# Détail du schéma SQLite — OmraVIP Quotes

> Complément à la **section 4** du prompt maître (`PROMPT_AGENT_OmraVIP.md`) et au fichier
> `database/schema.sql` (source de vérité du DDL).
> À lire intégralement par le sous-agent `database-sqlite` avant d'attaquer la **Phase 1**,
> et par `qa-financier` comme référence d'audit dès qu'une requête touche à ces tables.

---

## 1. Vue d'ensemble des groupes

| Groupe | Tables | Rôle |
|---|---|---|
| Configuration | `parametres_agence` | Ligne unique (id=1), réglages globaux |
| Référentiel | `clients` | Indépendant du cycle de vie d'un devis |
| **Dossier devis** | `devis` + 7 tables de détail | Le cœur métier |
| Catalogues | `catalogue_hotels`, `catalogue_compagnies` | Aide à la saisie uniquement — voir règle 2 |
| Séquençage | `compteurs_numerotation` | Numérotation atomique — voir `BACKEND_RUST_DETAIL.md` |

---

## 2. Le groupe « dossier devis »

`devis` est la table maître. Sept tables pendent d'elle en **`ON DELETE CASCADE`** :
`passagers`, `segments_vol`, `hebergements`, `transferts`, `train_haramain`,
`prestations_vip`. Ce sont des lignes de détail qui n'ont aucun sens hors du devis qui
les contient — supprimer un devis doit nettoyer tout son contenu sans laisser de résidus
orphelins.

`clients`, à l'inverse, est en **`ON DELETE RESTRICT`** vis-à-vis de `devis` : impossible
de supprimer un client tant qu'il a des devis rattachés — l'historique commercial prime
sur le confort de nettoyage.

---

## 3. Trois décisions de conception à ne jamais laisser dévier

Ce sont les points où un agent de développement pourrait, en toute bonne foi,
« normaliser » le schéma d'une façon qui casse une exigence métier implicite. Toute
migration future qui reviendrait sur l'une de ces trois règles doit être refusée par
`qa-financier`.

### 3.1 Les taux de change vivent dans `devis`, jamais dans une table à part

`taux_sar_dzd`, `taux_usd_dzd`, `taux_eur_dzd` sont des colonnes directes de `devis`,
volontairement dupliquées à chaque ligne plutôt que référencées via une table
`taux_historique` + clé étrangère.

**Pourquoi** : un devis est un document figé à sa date de création. Si le schéma évolue
vers une table de taux normalisée avec un `taux_id` en FK, un devis ancien se mettrait à
changer de valeur chaque fois que le taux du jour est mis à jour ailleurs — exactement ce
que la règle de verrouillage des taux (section 2 du prompt maître) interdit.

### 3.2 `nom_hotel` et `compagnie` sont du texte libre, jamais une FK vers les catalogues

`hebergements.nom_hotel` et `segments_vol.compagnie` ne référencent **pas**
`catalogue_hotels.id` / `catalogue_compagnies.id`.

**Pourquoi** : les catalogues ne servent qu'à l'autocomplete côté frontend au moment de
la saisie (voir `FRONTEND_VUE_DETAIL.md`, `catalogueStore`). Une fois la valeur copiée
dans la ligne du devis, elle devient indépendante. Si un hôtel du catalogue est renommé ou
supprimé six mois plus tard, les devis déjà émis ne doivent pas changer rétroactivement —
même logique que pour les taux de change.

### 3.3 Les totaux sont des colonnes stockées, jamais des vues calculées

`devis.cout_net_total`, `devis.marge_montant_total`, `devis.prix_vente_total` ne sont
**ni** une `VIEW` avec un `SUM()` SQL, **ni** une colonne générée
(`GENERATED ALWAYS AS`).

**Pourquoi** : SQLite n'a pas de type décimal natif ; un `SUM()` SQL retomberait sur de
l'arithmétique flottante — exactement ce que la règle « jamais de float sur l'argent »
interdit (voir section 2 du prompt maître et `qa-financier.md`). Ces colonnes sont donc
écrites explicitement par le backend Rust après un calcul en `Decimal`
(`calcul_prix.rs`, voir `BACKEND_RUST_DETAIL.md`), puis relues telles quelles.

Même logique pour deux autres colonnes :
- `hebergements.nb_nuitees` — INTEGER stocké, jamais un `julianday()` calculé à la volée
  en SQL.
- `passagers.alerte_passeport` — flag `0`/`1` stocké, recalculé par le backend
  (`alerte_passeport.rs`), jamais par une clause `WHERE date_expiration < date('now',
  '+6 months')` écrite directement en SQL — tentante, mais elle contournerait le point
  d'audit unique côté Rust.

---

## 4. Les contraintes CHECK comme « enum léger »

Plutôt que des tables de référence pour chaque liste fermée (`categorie` passager,
`type_vehicule`, `devise`, `statut`, `classe`…), le schéma utilise des
`CHECK (colonne IN (...))` directement sur la colonne concernée.

**Avantage** : pas de jointure supplémentaire pour des valeurs qui changent presque
jamais.

**Contrepartie à connaître** : SQLite ne permet pas de modifier une contrainte `CHECK`
existante avec un simple `ALTER TABLE`. Ajouter une valeur (par exemple un futur type de
véhicule) demande une reconstruction de table dans une migration numérotée
(`src-tauri/src/db/migrations/000N_xxx.sql`), pas un `ALTER TABLE ... ADD CONSTRAINT`.
C'est déjà documenté comme règle dans `.claude/agents/database-sqlite.md` — ce rappel
sert de référence croisée.

---

## 5. Index

Chaque table de détail porte un index sur son `devis_id`
(`idx_passagers_devis`, `idx_segments_devis`, `idx_heberg_devis`, `idx_transferts_devis`,
`idx_train_devis`, `idx_prestations_devis`), puisque la requête la plus fréquente de
l'application est « charger tout le contenu d'un devis ».

`devis` porte en plus trois index : `client_id` (historique par client), `numero_devis`
(recherche directe par référence), `statut` (filtre de `ListeDevisView`).

Toute nouvelle table de détail ajoutée par la suite doit suivre le même réflexe : un
index sur sa clé étrangère vers `devis` dès sa création, pas ajouté a posteriori une fois
la lenteur constatée.

---

## 6. `compteurs_numerotation` — la seule table sans lien au reste du schéma

`cle` (ex. `DEVIS-2026-07`) en clé primaire, `dernier_numero` en compteur. Aucune clé
étrangère : c'est un compteur pur, incrémenté de façon atomique via l'`UPSERT` détaillé
dans `BACKEND_RUST_DETAIL.md` (`numerotation.rs`).

Elle vit délibérément hors du groupe « dossier devis » : elle n'appartient à aucun devis
en particulier, elle appartient au mois. Ne jamais lui ajouter de FK vers `devis`.

---

## 7. Checklist de revue pour toute évolution de schéma

Avant qu'une migration touchant à ces tables soit considérée validée :

- [ ] Aucune colonne monétaire, de taux, ou de pourcentage n'est `REAL`/`INTEGER` (hors
      compteurs entiers légitimes comme `nb_nuitees` ou `categorie_etoiles`)
- [ ] Aucune nouvelle FK n'a été ajoutée entre une ligne de devis et une table catalogue
      (violerait la règle 3.2)
- [ ] Aucun total n'a été transformé en `VIEW` ou colonne générée (violerait la règle 3.3)
- [ ] Toute nouvelle valeur d'énumération passe par une migration numérotée, pas une
      modification directe de `database/schema.sql`
- [ ] Toute nouvelle table de détail rattachée à `devis` a son `ON DELETE CASCADE` et son
      index sur `devis_id`

---

## 8. Migration 0001 — Alignement schema.sql <-> modèles Rust (2026-07-21)

La migration `src-tauri/src/db/migrations/0001_align_tables_models_rust.sql` a été
générée pour aligner le schéma sur les structs Rust sous `src-tauri/src/models/`.

### Tableau d'état : Avant vs Après migration 0001

#### Table : parametres_agence

| Élément | Avant (schema.sql original) | Après (migration 0001) |
|---|---|---|
| `nom_agence` TEXT | → renommé **`nom_agence_fr`** pour correspondre au Rust `ParametresAgence::nom_agence_fr` |
| Champs manquants | `site_web`, `numero_agrement`, `devise_defaut`, `taux_tva` | Ajoutés (TEXT, pas nullable sauf taux_tva) |
| Legacy keep-alive | `marge_defaut_type`, `marge_defaut_valeur`, `devise_reference`, etc. | Conservés pour compatibilité progressive |

#### Table : clients

| Élément | Avant | Après |
|---|---|---|
| Champs manquants (Rust) | — | **`code_client TEXT UNIQUE NOT NULL`**, **`raison_sociale TEXT`**, **`nom_contact TEXT`**, **`pays TEXT`** (renommé de `wilaya`), **`type_client CHECK('particulier','agence')`**, **`created_at/updated_at`** |
| Ancien champ supprimé | `wilaya` | → renommé en **`pays`** |
| Notes | `notes TEXT` | → renommé **`remarques TEXT`** |

#### Table : devis

| Élément | Avant | Après |
|---|---|---|
| Champ renommé | `type_pelerinage TEXT ('Omra','Hadj')` | → **`type_visa TEXT CHECK('omra_standard','touristique','hadj')** aligné Rust |
| Champs manquants (Rust) | — | **`assurance_medicale BOOLEAN`**, **`devise_achat TEXT`**, **`remise TEXT`** |
| Enum statut | ('brouillon','envoye','confirme','annule') | → ('brouillon','finalise','envoye','accepte','perdu') aligné Rust `Devis::statut` |

#### Table : passagers

| Élément | Avant | Après |
|---|---|---|
| Colonnes supprimées | `civilite`, `nom`, `prenom` (séparés) | → fusionnés en **`nom_complet TEXT NOT NULL`** pour correspondre au Rust `Passager::nom_complet` |
| Champs manquants (Rust) | — | **`nationalite TEXT`**, **`lieu_delivrance TEXT`**, **`remarques TEXT`** |
| Conversion types | `DATE` (hint SQLite) | → **`TEXT`** explicite pour NaiveDate ↔ TEXT mapping correct |

#### Table : segments_vol

| Élément | Avant | Après |
|---|---|---|
| CHECK classe | ('economique','affaires','premium') | → corrigé en **('economique','affaires','premiere')** aligné Rust `SegmentVol::classe` |
| Champs manquants (Rust) | — | **`numero_vol TEXT`**, **`aeroport_depart TEXT`**, **`aeroport_arrivee TEXT`**, **`heure_depart TEXT HH:MM`**, **`heure_arrivee TEXT`**, **`remarques TEXT`** |
| Renommée | `devise TEXT` | → renommée **`devise_prix TEXT`** aligné Rust `SegmentVol::devise_prix` |

#### Table : hebergements

| Élément | Avant | Après |
|---|---|---|
| formule_repas | NOT NULL CHECK(...) | → **nullable** (Option<String> dans le modèle Rust) — CHECK conservé si non-null |
| Champs manquants (Rust) | — | **`nb_nuitees INTEGER`**, **`taxes_incluses BOOLEAN`**, **`remarques TEXT`** |
| Renommées | `prix_nuit`, `devise` | → **`prix_par_nuit TEXT`** et **`devise_prix TEXT`** aligné Rust |

#### Table : transferts

| Élément | Avant | Après |
|---|---|---|
| Champs manquants (Rust) | — | **`date_transfert TEXT`**, **`heure_transfert TEXT`**, **`nombre_vehicules INTEGER DEFAULT 1`**, **`remarques TEXT`** |
| Renommées | `prix`, `devise` | → **`prix_unitaire TEXT`** et **`devise_prix TEXT`** aligné Rust |

#### Table : prestations_vip

| Élément | Avant | Après |
|---|---|---|
| description | nullable TEXT | → **NOT NULL** (le modèle Rust `PrestationVip::description` est String non-nullable) |
| CHECK type_prestation | ('ziyara_privee','lounge_vip',...) | → corrigé en **('ziyarat','lounge','fast_track','bagages','zamzam','autre')** aligné Rust |
| Champs manquants (Rust) | — | **`quantite INTEGER DEFAULT 1`**, **`prix_unitaire TEXT`** (renommé de `prix`), **`devise_prix TEXT`** (renommé de `devise`), **`remarques TEXT`** |

#### Table : catalogue_hotels

| Élément | Avant | Après |
|---|---|---|
| categorie_etoiles INTEGER | → renommé en **`categorie TEXT`** avec valeurs '5_etoiles', '4_etoiles', etc. (Option<String> dans Rust) |
| Colonnes supplémentaires | — | **`adresse`, `site_web`, `remarques`, `actif BOOLEAN DEFAULT 1`** |

#### Table : catalogue_compagnies

| Élément | Avant | Après |
|---|---|---|
| Champs manquants (Rust) | — | **`code_iata TEXT`**, **`pays TEXT`**, **`site_web TEXT`**, **`actif BOOLEAN DEFAULT 1`** |

### Migration — Récapitulatif des opérations

Le fichier de migration `src-tauri/src/db/migrations/0001_align_tables_models_rust.sql`
applique les changements via le pattern safe (CREATE new → INSERT → DROP old → RENAME) :

1. **clients** : restructuration complète (code_client unique, raison_sociale, nom_contact, type_client CHECK, etc.)
2. **passagers** : fusion nom/civilite en nom_complet, ajout lieu_delivrance/remarques/nationalite, conversion types DATE→TEXT
3. **segments_vol** : ajout numero_vol/aeroport_depart/aeroport_arrivee/heure_depart/heure_arrivee/remarques, correction CHECK classe
4. **hebergements** : renommage prix_nuit→prix_par_nuit, devise→devise_prix, ajout nb_nuitees/taxes_incluses/remarques, formule_repas nullable
5. **transferts** : ajout date_transfert/heure_transfert/nombre_vehicules/remarques, renommage prix→prix_unitaire, devise→devise_prix
6. **prestations_vip** : correction CHECK type_prestation ('ziyarat'), description NOT NULL, ajout quantite/prix_unitaire/devise_prix/remarques
7. **devis** : renommage type_pelerinage→type_visa avec nouveau CHECK, ajout assurance_medicale/devise_achat/remise, correction enum statut
8. **parametres_agence** : renommage nom_agence→nom_agence_fr, ajout site_web/numero_agrement/devise_defaut/taux_tva
9. **catalogue_hotels** : renommage categorie_etoiles INTEGER→categorie TEXT, ajout adresse/site_web/remarques/actif
10. **catalogue_compagnies** : ajout code_iata/pays/site_web/contact/notes/actif

---

## 9. Commandes de validation SQLite

```bash
# Vérifier que le schema compile sans erreur
sqlite3 :memory: < database/schema.sql

# Inspecter chaque table alignée
sqlite3 :memory:.schema clients
sqlite3 :memory:.schema passagers
sqlite3 :memory:.schema segments_vol
sqlite3 :memory:.schema hebergements
sqlite3 :memory:.schema transferts
sqlite3 :memory:.schema prestations_vip
sqlite3 :memory:.schema devis
sqlite3 :memory:.schema parametres_agence
sqlite3 :memory:.schema catalogue_hotels
sqlite3 :memory:.schema catalogue_compagnies

# Vérifier les colonnes de chaque table (colonnes nommées)
sqlite3 -column -header database/omravip.db "SELECT * FROM sqlite_master WHERE type='table' ORDER BY name;"
```

### Script Python de comparaison automatique

Un script `docs/comparaison_schema_models.py` peut être créé pour :
1. Parser `PRAGMA table_info(nom_table)` pour chaque table dans le schema.sql
2. Parser les champs publics des structs Rust via regex sur les fichiers `src-tauri/src/models/*.rs`
3. Produire un diff colonne par colonne

---

## 10. Notes pour les agents suivants (non-database)

### Pour l'agent rust-backend-tauri

Les changements suivants doivent être appliqués côté Rust :

1. **Modèle `Devis`** : champ `type_pelerinage` renommé en `type_visa` (types différents)
2. **Modèle `SegmentVol::classe`** : valeur 'premium' corrigée en 'premiere'
3. **Modèle `PrestationVip::type_prestation`** : valeurs CHECK corrigées ('ziyatara_privee' → 'ziyarat')
4. **Modèle `Client`** : ajout des champs `code_client`, `raison_sociale`, `nom_contact`, `pays`, `type_client`, `created_at/updated_at`
5. **Modèle `ParametresAgence`** : renommage de certains champs (site_web, numero_agrement)
6. **Modèle `CatalogueHotel/CatalogueCompagnie`** : ajout des colonnes manquantes

### Pour l'agent vue-frontend

L'interface devra être mise à jour pour :
1. Le champ unique `nom_complet` du passager (remplace civilite/nom/prenom séparés)
2. Le nouveau code_client dans le formulaire client
3. Les enums de statut mis à jour
4. L'affichage potentiel de nouvelles colonnes (nb_nuitees, taxes_incluses, etc.)
