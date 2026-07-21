# Détail Frontend Vue.js — OmraVIP Quotes

> Complément à la **section 5** du prompt maître (`PROMPT_AGENT_OmraVIP.md`).
> À lire intégralement par le sous-agent `vue-frontend` avant d'attaquer la **Phase 5**
> (et la Phase 7 pour la partie dashboard marge).

---

## 1. Les 8 vues (routes)

| Vue | Rôle |
|---|---|
| `DashboardView` | Accueil : devis en cours, compteur d'alertes passeport actives, raccourci « Nouveau devis » |
| `NouveauDevisView` | Héberge le wizard — orchestre les 7 composants `devis/*` (section 3) |
| `ListeDevisView` | Tableau des devis, filtrable par statut/client/date, badge alerte passeport visible en colonne |
| `ClientsView` | CRUD clients |
| `CatalogueHotelsView` / `CatalogueVolsView` | CRUD des catalogues réutilisables (accélère la saisie) |
| `ParametresView` | Marge par défaut, logo, coordonnées agence |
| `MargeDashboardView` | Vue **interne** — agrégats de marge par période, jamais exposée côté client |

---

## 2. Les 4 stores Pinia

### `devisStore`
Le cœur de l'application. Contient l'état complet du devis en cours de saisie et l'étape
active du wizard. Tous les formulaires du wizard lisent et écrivent exclusivement dans ce
store — jamais d'état local dupliqué dans un composant.

```typescript
interface DevisStoreState {
  devisId: number | null;          // null tant que le devis n'est pas encore sauvegardé
  etapeActive: EtapeWizard;        // 'passagers' | 'vols' | 'hebergement' | 'transferts' | 'prestations' | 'financier' | 'recapitulatif'
  clientId: number | null;
  typePelerinage: 'Omra' | 'Hadj';
  dateDepart: string | null;
  dateRetour: string | null;
  passagers: Passager[];
  segmentsVol: SegmentVol[];
  hebergements: Hebergement[];
  transferts: Transfert[];
  trainHaramain: TrainHaramain[];
  prestationsVip: PrestationVip[];
  tauxChange: TauxChange | null;   // verrouillé côté backend à la sauvegarde
  margeType: 'pourcentage' | 'montant_fixe';
  margeValeur: string;             // chaîne décimale, jamais number
  totaux: TotauxDevis | null;      // renvoyé par le backend après recalcul, jamais recalculé en JS
}
```

Actions principales : `ajouterPassager`, `ajouterSegmentVol`, `sauvegarderBrouillon`
(persiste l'état actuel via une command Tauri sans exiger que le wizard soit complet),
`validerEtape`, `recalculerTotaux` (appelle le backend, ne calcule rien localement),
`genererDevis` (déclenche la création définitive + numérotation).

### `clientStore`
Liste et recherche de clients, avec une action `creerClientRapide` utilisable directement
depuis l'étape « Passagers » du wizard, sans quitter le flux de saisie.

### `catalogueStore`
Hôtels et compagnies aériennes, utilisé pour l'autocomplete dans `FormulaireHebergement`
et `FormulaireVols`.

### `devisesStore`
Taux de change saisis pour le devis en cours. Ne gère que l'état avant verrouillage — une
fois le devis sauvegardé, les taux verrouillés vivent dans `devisStore.tauxChange` et ne
doivent plus être modifiés par ce store.

---

## 3. Les 8 composants du wizard (`components/devis/`)

Ordre d'affichage = ordre métier. Chaque formulaire émet un événement `valide` quand
l'étape est complète, ce qui active le passage à l'étape suivante dans `NouveauDevisView`.

### `FormulairePassagers.vue`
Ajout dynamique de passagers par catégorie (`adulte`, `enfant_avec_lit`,
`enfant_sans_lit`, `bebe`), saisie passeport (numéro + date d'expiration), sélection du
type de visa et saisie du coût d'assurance. Affiche `AlertePasseport.vue` en ligne pour
chaque passager dès que la date d'expiration est renseignée.

### `FormulaireVols.vue`
Liste ordonnée et réordonnable de segments de vol (multi-destinations). Chaque segment :
compagnie, classe, date, origine/destination, et **3 champs de prix** (adulte / enfant /
bébé) + devise. Prévoir un bouton « dupliquer ce segment » pour accélérer la saisie du
trajet retour.

### `FormulaireHebergement.vue`
Deux blocs (Makkah / Médine). Sélection hôtel via autocomplete sur `catalogueStore`,
saisie des dates check-in/check-out. Affiche le nombre de nuitées en temps réel via
`useCalculNuitees` — **affichage indicatif uniquement** : la valeur canonique qui part en
base est recalculée côté Rust à la sauvegarde (voir `BACKEND_RUST_DETAIL.md`).

### `FormulaireTransferts.vue`
4 lignes de transferts obligatoires pré-remplies avec les trajets standards, + jusqu'à
2 lignes optionnelles ajoutables. Sélection du véhicule (`GMC_Yukon`,
`Mercedes_Classe_E`, `Bus_VIP_prive`) par menu déroulant. Section dédiée pour le train
Haramain (trajet + classe).

### `FormulairePrestationsVIP.vue`
Ajout de lignes libres pour ziyara privée (avec option guide/véhicule), lounge VIP,
fast-track, bagages, eau Zamzam, et un champ libre « autre ».

### `FormulaireFinancier.vue`
Saisie des taux du jour (SAR/USD/EUR → DZD), choix du type de marge (%/montant fixe).
Affiche un calcul en direct à titre indicatif. **Vue strictement interne** — jamais
accessible depuis un parcours ou un export destiné au client.

### `RecapitulatifDevis.vue`
Dernière étape avant validation. Bascule « Vue client » / « Vue interne » pour
prévisualiser exactement ce qui partira en PDF avant de déclencher la génération.

### `AlertePasseport.vue`
Composant réutilisable (badge rouge/orange/vert). Reçoit en prop directement le flag
`alerte_passeport` calculé côté Rust — **aucun recalcul de date en JavaScript**, pour
garder une source unique de vérité.

---

## 4. Composables

Ce sont des wrappers d'**affichage temps réel**, pas des moteurs de calcul. Ils donnent un
retour visuel instantané pendant la saisie, mais toute valeur réellement stockée est
recalculée côté backend au moment de la sauvegarde.

```typescript
function useCalculNuitees(
  checkin: Ref<string | null>,
  checkout: Ref<string | null>
): ComputedRef<number | null>

function useConversionDevises(
  montant: Ref<string>,
  devise: Ref<Devise>,
  taux: TauxChange
): ComputedRef<string>   // retourne une chaîne décimale formatée pour l'affichage

function useValidationPasseport(
  dateExpiration: Ref<string | null>,
  dateRetour: Ref<string | null>,
  seuilMois?: number  // défaut 6, alignable sur parametres_agence
): ComputedRef<boolean>
```

---

## 5. Point d'attention i18n / RTL

Au-delà du PDF (traité par l'agent `pdf-generator`), si des libellés métier sont affichés
en arabe à l'écran, la bascule RTL doit se faire **par section**, pas globalement sur
`document.dir` — sinon un champ de saisie en français à côté d'un libellé arabe hérite
d'une direction incorrecte. Utiliser `dir="rtl"` localement sur les conteneurs concernés,
piloté par `vue-i18n` selon la locale active.
