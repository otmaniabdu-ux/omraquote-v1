# Détail Backend Rust — Services de calcul — OmraVIP Quotes

> Complément à la **section 6** du prompt maître (`PROMPT_AGENT_OmraVIP.md`).
> À lire intégralement par le sous-agent `rust-backend-tauri` avant d'attaquer la
> **Phase 3**, et par `qa-financier` comme référence d'audit.

---

## Principe de conception

Chaque service sous `src-tauri/src/services/` est une **fonction pure au maximum** : elle
prend des structs en entrée, retourne un résultat, et ne touche à SQLite que si c'est
strictement nécessaire (un seul service en a réellement besoin — voir `numerotation.rs`
plus bas). C'est ce qui les rend testables unitairement sans base de données.

---

## `conversion_devises.rs`

Point d'entrée **unique** pour toute conversion vers DZD. Aucun autre module du projet ne
doit multiplier un montant par un taux directement — tout passe par ici, pour garder un
seul point d'audit.

```rust
pub struct TauxChange {
    pub sar_dzd: Decimal,
    pub usd_dzd: Decimal,
    pub eur_dzd: Decimal,
}

pub fn convertir_vers_dzd(montant: Decimal, devise_source: Devise, taux: &TauxChange) -> Decimal {
    match devise_source {
        Devise::Dzd => montant,
        Devise::Sar => montant * taux.sar_dzd,
        Devise::Usd => montant * taux.usd_dzd,
        Devise::Eur => montant * taux.eur_dzd,
    }
}
```

---

## `calcul_prix.rs` — l'agrégateur central

Prend un `DevisComplet` (tous les passagers, segments, hébergements, transferts,
prestations déjà chargés) et le `TauxChange` verrouillé, retourne :

```rust
pub struct TotauxDevis {
    pub cout_net_total: Decimal,
    pub marge_montant_total: Decimal,
    pub prix_vente_total: Decimal,
}
```

### Piège métier à ne pas rater : mapping catégories passager → paliers tarifaires

Les passagers ont **4 catégories** (`adulte`, `enfant_avec_lit`, `enfant_sans_lit`,
`bebe`), mais les vols et le train Haramain n'ont que **3 paliers de prix**
(adulte/enfant/bébé — les compagnies ne distinguent pas avec/sans lit pour la
tarification aérienne). Un mapping explicite est obligatoire :

```rust
fn categorie_vers_tarif_vol(cat: CategoriePassager) -> TarifAge {
    match cat {
        CategoriePassager::Adulte => TarifAge::Adulte,
        CategoriePassager::EnfantAvecLit | CategoriePassager::EnfantSansLit => TarifAge::Enfant,
        CategoriePassager::Bebe => TarifAge::Bebe,
    }
}
```

Sans ce mapping explicite, deux erreurs typiques : ignorer les enfants sans lit dans le
calcul du prix des vols, ou inventer un 4ᵉ palier tarifaire qui n'existe pas chez les
compagnies aériennes.

### Règle d'arrondi

La marge s'applique **une seule fois, à la fin, sur le total agrégé** — jamais ligne par
ligne. Appliquer la marge poste par poste puis sommer produit des écarts d'arrondi
cumulés par rapport à une application unique sur le total.

---

## `calcul_nuitees.rs`

```rust
pub fn calculer_nuitees(checkin: NaiveDate, checkout: NaiveDate) -> Result<i64, ErreurMetier> {
    if checkout <= checkin {
        return Err(ErreurMetier::DatesIncoherentes);
    }
    Ok((checkout - checkin).num_days())
}
```

**Cas de test obligatoires** (à couvrir par `qa-financier`) : un séjour traversant une fin
de mois (ex. 30 → 2 jours suivants), et un séjour traversant février d'une année
bissextile. Un calcul artisanal en JavaScript ou une manipulation de date manuelle se
trompe facilement sur ces deux cas.

---

## `alerte_passeport.rs`

```rust
pub fn expire_avant_seuil(date_expiration: NaiveDate, date_retour: NaiveDate, seuil_mois: u32) -> bool {
    let seuil = date_retour.checked_add_months(Months::new(seuil_mois))
        .expect("date hors limites");
    date_expiration < seuil
}
```

Cas de test obligatoires : exactement au seuil, seuil moins un jour, seuil plus un jour.

---

## `numerotation.rs` — le seul service qui touche la base

Seul cas où une fonction pure ne suffit pas : il faut garantir qu'aucun numéro n'est
distribué deux fois si deux devis sont créés au même instant. La bonne approche est une
requête SQLite **atomique**, jamais un lire-puis-écrire séparé (vulnérable à une
condition de course) :

```sql
INSERT INTO compteurs_numerotation (cle, dernier_numero) VALUES (?1, 1)
ON CONFLICT(cle) DO UPDATE SET dernier_numero = dernier_numero + 1
RETURNING dernier_numero;
```

```rust
pub fn generer_numero_devis(conn: &Connection, date: NaiveDate) -> Result<String, ErreurMetier> {
    let cle = format!("DEVIS-{}-{:02}", date.year(), date.month());
    let numero: i64 = conn.query_row(
        "INSERT INTO compteurs_numerotation (cle, dernier_numero) VALUES (?1, 1)
         ON CONFLICT(cle) DO UPDATE SET dernier_numero = dernier_numero + 1
         RETURNING dernier_numero",
        params![cle],
        |row| row.get(0),
    )?;
    Ok(format!("{}-{:03}", cle, numero))
}
```

C'est précisément le point que `qa-financier` doit vérifier en fin de Phase 3 : rejeter
toute implémentation qui ferait un `SELECT` suivi d'un `UPDATE` en deux requêtes séparées.

---

## `generation_pdf.rs`

Ne fait pas le rendu lui-même — le template Typst est la responsabilité de l'agent
`pdf-generator`. Ce service assemble le `DevisComplet` + `TotauxDevis`, sélectionne la
variante (client ou interne), et invoque la compilation Typst. C'est la couture entre les
deux agents : ce fichier ne doit contenir aucune logique de mise en page, uniquement
l'assemblage des données à injecter dans le template.

---

## `utils/decimal_helpers.rs`

Deux fonctions à centraliser ici pour éviter toute réécriture dispersée :

- **`arrondir_2_decimales`** : arrondi centralisé, stratégie
  `RoundingStrategy::MidpointAwayFromZero` (équivalent `ROUND_HALF_UP`). Ne jamais
  réimplémenter cette règle localement dans un autre service.
- **Parseur d'entrée utilisateur tolérant à la virgule décimale française** —
  `"125,50"` → `Decimal`. La saisie clavier en français (utilisée par l'agence) produit
  naturellement une virgule et non un point ; un parseur ou une regex copiés d'un exemple
  anglophone par défaut casseront silencieusement sur cette entrée.

```rust
pub fn parser_montant_utilisateur(saisie: &str) -> Result<Decimal, ErreurMetier> {
    let normalise = saisie.trim().replace(',', ".");
    Decimal::from_str(&normalise).map_err(|_| ErreurMetier::MontantInvalide)
}
```

---

## Rappel pour l'audit `qa-financier`

Chaque fonction ci-dessus doit avoir son test unitaire correspondant sous
`#[cfg(test)]` dans le même fichier, couvrant explicitement les cas limites mentionnés.
L'absence de test sur un cas limite listé dans ce document est à traiter comme une
non-conformité bloquante avant de clore la Phase 3.

---

## Correctifs post-revue (2026-07-21)

### 1. `commands/mod.rs` — nettoyage et extension

**Problème :** Fichier corrompu avec des lignes orphelines hors module
(`use commands::marge::{...}`, identifiants `get_statistiques`/`get_top_clients`).
Modules manquants déclarés dans les imports de `main.rs` mais absents.

**Correctif :** Supprime toutes les lignes orphelines (imports, identifiants) du mod.rs.
Ne conserve que les déclarations `pub mod`. Ajoute les modules manquants :
`tansferts`, `prestations`, `pdf`, `parametres`.

### 2. `services/mod.rs` — incohérence de nommage

**Problème :** Déclaration `pub mod validations;` mais fichier réel = `validation.rs`
(singulier). Toutes les importations croisées (`devis.rs:10-11`, `hebergements.rs:3`)
échouaient en compilation.

**Correctif :** Renomme la déclaration en `pub mod validation;` (cohérent avec le nom du fichier).

### 3. `commands/devis.rs` — corrections critiques

| Ligne(s) | Problème | Correctif |
|----------|----------|-----------|
| 47-48 | Brace fermante manquante après `get_alertes_tous_devis` | Ajout du `}` manquant |
| 84-109 | Fragments de migration (`generer_numero_devis`) collés hors-fonction | Suppression complète (fonction existante dans `services/numerotation.rs`) |
| 113-114 | Double `#[tauri::command]` sur `create_devis` | Conservation d'un seul attribut |
| 315-316 | Brace fermante orpholine après `get_devis_by_id(state, id)` dans `update_devis` | Retrait de la brace ; la fonction se termine correctement par le retour |
| 10-11, 3 | Imports `crate::services::validations::...` (pluriel) | Corrigé vers `crate::services::validation::...` (singulier) |

### 4. Structs de création/mise à jour manquantes

Les structs suivantes ont été ajoutées pour que les commandes Tauri et le frontend
puissent échanger des payloads typés :

| Struct | Fichier | Usage |
|--------|---------|-------|
| `HebergementCreate` | `models/hebergement.rs` | Création d'un hébergement lié à un devis (exclut id/updated_at) |
| `TransfertCreate` | `models/transfert.rs` | Création d'un transfert lié à un devis |
| `TransfertUpdate` | `models/transfert.rs` | Mise à jour partielle de transfert (tous les champs Option) |
| `PrestationVipCreate` | `models/prestation_vip.rs` | Création d'une prestation VIP liée à un devis |
| `PrestationVipUpdate` | `models/prestation_vip.rs` | Mise à jour partielle de prestation VIP (tous les champs Option) |

> **Note :** `DevisCreate` et `DevisUpdate` existaient déjà dans `models/devise.rs`. Le nom
> du module (`devise`) correspond à la devise (monnaie). Les types de devis se trouvent dans
> `models/devis.rs` (avec un 's'). Cette distinction est intentionnelle pour ne pas confondre les
> entités métier.

### 5. CRUD manquants implémentés

| Fichier | Commandes ajoutées |
|---------|-------------------|
| `commands/hebergements.rs` | `create_hebergement`, `get_hebergement_by_id`, `list_hebergements_by_devis`, `update_hebergement`, `delete_hebergement` |
| `commands/transferts.rs` (était vide) | `create_transfert`, `get_transfert_by_id`, `list_transferts_by_devis`, `update_transfert`, `delete_transfert` |
| `commands/prestations.rs` (était vide) | `create_prestation`, `get_prestation_by_id`, `list_prestations_by_devis`, `update_prestation`, `delete_prestation` |

Chaque CRUD suit le pattern existant dans `clients.rs` : état Tauri via `State<DbState>`,
requêtes préparées avec `params!`, erreurs retournées via `Result<T, String>`.

### 6. `utils/mod.rs` — module réactivé

**Problème :** Module déclaré mais `mod.rs` vide (pas de `pub mod decimal_helpers;`).
Les autres modules ne pouvaient pas importer les helpers décimaux.

**Correctif :** Ajout de `pub mod decimal_helpers;` dans `utils/mod.rs`.

### 7. `main.rs` — imports manquants

**Problème :** Trois commandes Tauri étaient inscrites dans le handler (`invoke_handler`)
mais non importées :
- `get_alertes_devis` (dans `commands::devis`)
- `generate_pdf_client` (dans `commands::pdf`)
- `generate_pdf_interne` (dans `commands::pdf`)

**Correctif :** Ajout de `get_alertes_devis` dans l'import existant `commands::devis`, et
ajout d'un import dédié pour les deux fonctions PDF :
```rust
use commands::pdf::{
    generate_pdf_client, generate_pdf_interne,
};
```

---

## Commandes Tauri exposées

| Commande IPC | Fonction Rust | Module |
|-------------|---------------|--------|
| `create_client` | `commands::clients::create_client` | clients |
| `get_client_by_id` | `commands::clients::get_client_by_id` | clients |
| `list_clients` | `commands::clients::list_clients` | clients |
| `update_client` | `commands::clients::update_client` | clients |
| `delete_client` | `commands::clients::delete_client` | clients |
| `generate_client_code` | `commands::clients::generate_client_code` | clients |
| `create_devis` | `commands::devis::create_devis` | devis |
| `get_devis_by_id` | `commands::devis::get_devis_by_id` | devis |
| `list_devis` | `commands::devis::list_devis` | devis |
| `update_devis` | `commands::devis::update_devis` | devis |
| `delete_devis` | `commands::devis::delete_devis` | devis |
| `calculate_totals` | `commands::devis::calculate_totals` | devis |
| `get_alertes_devis` | `commands::devis::get_alertes_devis` | devis |
| `create_passager` | `commands::passagers::create_passager` | passagers |
| `get_passager_by_id` | `commands::passagers::get_passager_by_id` | passagers |
| `list_passagers_by_devis` | `commands::passagers::list_passagers_by_devis` | passagers |
| `delete_passager` | `commands::passagers::delete_passager` | passagers |
| `valider_dates_devis_command` | `commands::validation::valider_dates_devis_command` | validation |
| `valider_hebergement_command` | `commands::validation::valider_hebergement_command` | validation |
| `get_passeport_alertes` | `commands::validation::get_passeport_alertes` | validation |
| `check_passager_passeport` | `commands::validation::check_passager_passeport` | validation |
| `generate_pdf_client` | `commands::pdf::generate_pdf_client` | pdf |
| `generate_pdf_interne` | `commands::pdf::generate_pdf_interne` | pdf |
| `create_hotel` | `commands::hotels::create_hotel` | hotels |
| `get_hotel_by_id` | `commands::hotels::get_hotel_by_id` | hotels |
| `list_hotels` | `commands::hotels::list_hotels` | hotels |
| `update_hotel` | `commands::hotels::update_hotel` | hotels |
| `delete_hotel` | `commands::hotels::delete_hotel` | hotels |
| `create_compagnie` | `commands::compagnies::create_compagnie` | compagnies |
| `get_compagnie_by_id` | `commands::compagnies::get_compagnie_by_id` | compagnies |
| `list_compagnies` | `commands::compagnies::list_compagnies` | compagnies |
| `update_compagnie` | `commands::compagnies::update_compagnie` | compagnies |
| `delete_compagnie` | `commands::compagnies::delete_compagnie` | compagnies |
| `create_hebergement` | `commands::hebergements::create_hebergement` | hebergements |
| `get_hebergement_by_id` | `commands::hebergements::get_hebergement_by_id` | hebergements |
| `list_hebergements_by_devis` | `commands::hebergements::list_hebergements_by_devis` | hebergements |
| `update_hebergement` | `commands::hebergements::update_hebergement` | hebergements |
| `delete_hebergement` | `commands::hebergements::delete_hebergement` | hebergements |
| `create_transfert` | `commands::transferts::create_transfert` | transferts |
| `get_transfert_by_id` | `commands::transferts::get_transfert_by_id` | transferts |
| `list_transferts_by_devis` | `commands::transferts::list_transferts_by_devis` | transferts |
| `update_transfert` | `commands::transferts::update_transfert` | transferts |
| `delete_transfert` | `commands::transferts::delete_transfert` | transferts |
| `create_prestation` | `commands::prestations::create_prestation` | prestations |
| `get_prestation_by_id` | `commands::prestations::get_prestation_by_id` | prestations |
| `list_prestations_by_devis` | `commands::prestations::list_prestations_by_devis` | prestations |
| `update_prestation` | `commands::prestations::update_prestation` | prestations |
| `delete_prestation` | `commands::prestations::delete_prestation` | prestations |
| `get_statistiques` | `commands::marge::get_statistiques` | marge |
| `get_top_clients` | `commands::marge::get_top_clients` | marge |

---

## Correctif post-revue (2026-07-21) -- Problèmes 1 et 2

### 1. Feature flag `pdf-generation`

**Problème :** Les dépendances Typst (`typst`, `typst-pdf`, `comemo`, `ecow`) n'étaient pas dans
`Cargo.toml`, mais `services/generation_pdf.rs` les importait, empêchant toute compilation.

**Architecture adoptée (Option A + gating) :**

- **`Cargo.toml`** : dépendances optionnelles sous `[features] pdf-generation = [...]`.
- **`services/mod.rs`** : `#[cfg(feature = "pdf-generation")] pub mod generation_pdf;`
- **`commands/pdf.rs`** : deux blocs conditionnels (`#[cfg(feature)]` / `#[cfg(not(feature))`) -- le premier contient l'implémentation réelle, le second des fonctions de repli qui retournent une erreur informative.
- **`main.rs`** : les imports et `invoke_handler` utilisent toujours les noms `generate_pdf_client` / `generate_pdf_interne` (toujours présents grâce au fallback), donc la commande Tauri est toujours enregistrée mais fonctionne différemment selon le flag.

**Schéma d'import :**
```
Cargo.toml         → feature "pdf-generation" active/optionnelle
services/mod.rs    → #[cfg(feature)] mod generation_pdf
commands/pdf.rs    → #[cfg] impl réelle + #[cfg(not)] fallback (erreurs explicites)
main.rs            → imports conditionnels mais commandes toujours dans invoke_handler
```

**Compiler :**
```bash
# Avec PDF
cargo check --features pdf-generation --all-targets

# Sans PDF (par défaut)
cargo check --all-targets
```

**Activer le PDF par défaut :** Supprimer la section `[features]` de `Cargo.toml`, dé-commenter les dépendances Typst, retirer tous les `#[cfg(feature = "pdf-generation")]` du code source. Recommandé uniquement quand l'équipe est prête à inclure ces crates dans le build par défaut.

### 2. Schéma d'import `alerte_passeport` -- source unique

**Problème :** La fonction `alerte_passeport` était dupliquée dans deux fichiers
(`services/validation.rs` et `services/alerte_passeport.rs`).

**Architecture adoptée :**

```
services/
  alerte_passeport.rs   ← source canonique (définition + tests complets)
  validation.rs         → pub use crate::services::alerte_passeport::alerte_passeport;
                         → (re-export pour rétrocompatibilité des imports croisés)

Importeurs :
  commands/devis.rs          → use crate::services::validation::alerte_passeport;  (via re-export)
  services/validation.rs    → pub use ... (même chemin)
```

**Règle :** Toute nouvelle implémentation de la logique d'alerte passeport doit être ajoutée dans
`alerte_passeport.rs`. `validation.rs` ne contient plus que les fonctions de validation générique et le re-export.

---
