# OmraVIP Quotes

Application de bureau pour la gestion des devis VIP Omra/Hadj — **El Mouhssinouen Tours**.

Générez, suivez et exportez en PDF les devis de voyage pèlerinage, avec calcul précis des coûts, marges et conversion de devises.

---

## Fonctionnalités

- **Devis multi-étapes** : 7 étapes guidées (passagers, vols, hébergement, transferts, prestations VIP, financier, récapitulatif)
- **Calcul automatique des prix** : conversion SAR/USD/EUR → DZD avec taux verrouillés par devis
- **Marge dynamique** : pourcentage ou montant fixe, vue interne uniquement (jamais exposée au client)
- **Export PDF bilingue** : variantes client (prix de vente) et interne (coût + marge), texte français/arabe RTL via Typst
- **Gestion des alertes passeport** : vérification automatique de la validité (≥ 6 mois après retour)
- **Catalogue hôtels & compagnies** : autocomplétion pour accélérer la saisie
- **Numérotation atomique** : les numéros de devis ne sont jamais dupliqués, même en accès simultané
- **Bilingue FR/AR** : interface complète en français et arabe avec support RTL par section
- **Données locales** : base SQLite intégrée, aucune connexion serveur requise

## Stack technique

| Couche | Technologies |
|---|---|
| **Frontend** | Vue 3 + TypeScript, Pinia, vue-i18n, vue-router, Chart.js |
| **Backend** | Rust, Tauri 2.0, rusqlite, rust_decimal, chrono |
| **Base de données** | SQLite (fichier local intégré) |
| **PDF** | Typst (typst + typst-pdf), polices embarquées dans le binaire |

## Architecture

```
src/                          # Vue.js frontend
  stores/                     # Pinia (devis, client, catalogue)
  views/                      # Pages (Dashboard, ListeDevis, NouveauDevis…)
  components/devis/           # Composables du wizard 7 étapes
  composables/                # Calcul nuitées, conversion devises, validation passeport
  types/                      # Interfaces TypeScript synchronisées avec Rust
  locales/                    # i18n FR/AR
router/index.ts               # Routes client-side

src-tauri/                    # Rust backend (Tauri 2.0)
  src/commands/               # Endpoints IPC Tauri (CRUD, PDF, stats)
  src/services/               # Logique métier (prix, devises, marge, numerotation)
  src/models/                 # Structs Rusqlite avec serde
  src/db/                     # Connexion SQLite + migrations numérotées
  capabilities/default.json   # Permissions Tauri
  tauri.conf.json             # Métadonnées, window, bundle (msi, appimage)
  Cargo.toml                  # Dépendances Rust (rust_decimal, chrono, …)

database/
  schema.sql                   # Source de vérité du schéma SQLite
docs/                         # Documentation technique détaillée
templates/devis_pdf.typ       # Template Typst pour les PDF de devis
```

**Principe d'architecture** : le frontend appelle des commandes Tauri via `invoke()`. Les commandes déléguent à des services purs (sans accès DB direct). Les stores Pinia contiennent uniquement les données — zéro logique financière dans le frontend.

## Installation

### Prérequis

- **Node.js** ≥ 18
- **Rust** ≥ 1.70 (rustup)
- **Tauri CLI** : `npm install -g @tauri-apps/cli`
- **Windows** : [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ workload)
- **Linux** : `sudo apt install libsqlite3-dev libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev`

### Configuration du projet

```bash
# Cloner le dépôt
git clone https://github.com/otmaniabdu-ux/omraquote-v1.git
cd omraquote-v1

# Installer les dépendances frontend
npm install

# Démarrer le serveur de développement (Vite + Tauri HMR)
npm run tauri dev

# Build production (MSI sur Windows, AppImage sur Linux)
npm run tauri build
```

### Exécution uniquement frontend (débogage)

```bash
npm install
npm run dev        # http://localhost:1420
npm run preview    # serveur standalone du build
```

## Commandes disponibles

| Commande | Description |
|---|---|
| `npm run dev` | Vite dev server (port 1420) |
| `npm run tauri dev` | Full-stack (Rust + Vue HMR) |
| `npm run build` | TypeScript type-check + build Vite |
| `npm run tauri build` | Build Tauri (.msi / .AppImage) |
| `npm run preview` | Preview du build frontend |

## Règles métier clés

- **Tous les montants** utilisent `rust_decimal::Decimal` — jamais de flottant
- **Taux de change** : verrouillés à la création du devis, ne changent jamais rétroactivement
- **Noms des hôtels et compagnies** : texte libre dans un devis (catalogue = autocomplétion uniquement)
- **Totaux stockés** en base (`cout_net_total`, `marge_montant_total`, `prix_vente_total`) — jamais calculés par requête SQL
- **PDF client vs interne** : deux templates séparés, deux fonctions Rust distinctes. Les données de coût/marge n'existent jamais dans le payload compilé pour la variante client.

## Statut du devis

```
brouillon → finalise → envoye → accepte  /  perdu
```

## Documentation technique

| Fichier | Contenu |
|---|---|
| `CLAUDE.md` | Guide pour Claude Code — architecture, règles agents, concepts métier |
| `PROMPT_AGENT_OmraVIP.md` | Prompt maître de développement multi-phases |
| `docs/DATABASE_SCHEMA_DETAIL.md` | Schéma DB détaillé, règles de conception, migration 0001 |
| `docs/BACKEND_RUST_DETAIL.md` | Services de calcul, commandes Tauri, feature flags |
| `docs/FRONTEND_VUE_DETAIL.md` | Vues, stores Pinia, composants wizard, composables |
| `docs/PDF_GENERATION_DETAIL.md` | Architecture PDF Typst, polices, séparation client/interne |

## Sous-agents spécialisés

Le projet est structuré autour de 5 agents spécialisés (`.claude/agents/`) :

- **`rust-backend-tauri`** — Commands IPC, modèles Rusqlite, services de calcul
- **`vue-frontend`** — Vues, composants wizard, stores Pinia, i18n FR/AR
- **`database-sqlite`** — Schéma, migrations, requêtes complexes
- **`pdf-generator`** — Templates Typst, polices embarquées, rendu RTL
- **`qa-financier`** — Audit des modules financiers (totaux, nuitées, devises, passeport)

## Licence

Copyright © El Mouhssinouen Tours — Tous droits réservés.

---

### Dernières Mises à Jour (Juillet 2026)
- **Corrections de Stabilité** : Correction des crashs de parsing de base de données (conversion `NaiveDate` vers `NaiveDateTime` pour les dates système) et sécurisation du backend Tauri.
- **Résolution des pertes de données** : Intégration complète des données financières, des trains Haramain, des prestations VIP personnalisées et rétablissement de la réactivité sur les formulaires d'hébergement et de vol.
- **Correction Compilation** : Résolution du problème de compilation Rust en activant les fonctionnalités `chrono` et `rust_decimal` pour `rusqlite`.
- **Refactoring Architectural (Phase 3)** : Centralisation complète de toutes les requêtes SQLite brutes dans une couche de services DB dédiée (`src-tauri/src/services/db/`), harmonisation de la propagation des erreurs via `AppError`/`AppResult` et nettoyage des contrôleurs IPC Tauri.


