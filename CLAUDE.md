# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**OmraVIP Quotes** is a Tauri 2.0 desktop application for "El Mouhssinouen Tours", managing omra/hajj travel quotes (devis). Desktop-first — it ships as Windows (.msi) and Linux (.AppImage) installers via the `bundle` config.

- **Frontend**: Vue 3 + TypeScript, Vite, Pinia stores, vue-i18n (FR/AR), vue-router
- **Backend**: Rust / Tauri 2.0 with rusqlite (SQLite bundled), rust_decimal for all monetary values
- **Data**: Local SQLite database (`database/schema.sql`) — no server

## Key Directories

```
src/                          # Vue.js frontend
  stores/                     # Pinia stores (devisStore, clientStore, catalogueStore)
  views/                      # Page-level components (Dashboard, MargeDashboard, CatalogueHotels/Vols)
  components/devis/           # Multi-step quote wizard sub-components
  composables/                # Shared Vue composables (calculNuitees, conversionDevises, validationPasseport)
  types/devis.types.ts        # TypeScript interfaces — must match Rust struct serialised names
  locales/fr.json, ar.json    # i18n translations
router/index.ts               # Client-side routes
database/schema.sql           # Single source of truth for DB schema
src-tauri/                    # Rust backend (Tauri)
  src/commands/               # Tauri IPC handler functions (CRUD, PDF generation)
  src/services/               # Business logic (calcul_prix, conversion_devises, alerte_passeport, numerotation)
  src/models/                 # Rusqlite model structs with serde
  src/db/                     # Connection and migrations
  capabilities/default.json   # Tauri capability config (permissions)
  tauri.conf.json             # App metadata, window config, bundle targets
templates/devis_pdf.typ       # Typst PDF template for quote generation
```

## Architecture

- **Tauri commands** (`src-tauri/src/commands/`) are the only IPC bridge. The frontend calls them via `invoke()` from `@tauri-apps/api/core`.
- **Services layer** (`src-tauri/src/services/`) owns all business logic: pricing, currency conversion, margin calculation, passport alerts, night count calculation, numbering. Commands delegate to services.
- **Stores** (`src/stores/`) hold app state in Pinia — no business logic in stores or components, only data fetching and display formatting.
- **Types** (`src/types/devis.types.ts`) are hand-synced with Rust structs. Field names must match exactly for deserialization.
- The quote wizard follows a 7-step sequence: Passagers → Vols → Hébergement → Transferts → Prestations VIP → Financier → Récapitulatif (see `NouveauDevisView.vue`).

## Development Commands

```bash
# Frontend dev server (port 1420)
npm run dev

# Full-stack Tauri dev (Rust + Vite HMR)
npm run tauri dev

# TypeScript type-check + production frontend build
npm run build

# Production Tauri app build (.msi on Windows, .AppImage on Linux)
npm run tauri build

# Preview built frontend (standalone)
npm run preview
```

## Rules from existing agent specs

The repo includes developer-facing agent specs in `.claude/agents/`. Their rules are non-negotiable for this project:

### rust-backend-tauri (`agent spec`)
- **Never** use `f32`/`f64` for money, exchange rates, or percentages — use `rust_decimal::Decimal` everywhere.
- Dates: always `chrono::NaiveDate`. Never float-based date math.
- Monetary columns in SQLite are `TEXT` (decimal string), never `REAL`.
- Every Tauri command must return typed errors (`thiserror`), never `panic!` on user input.
- All business logic lives in `src-tauri/src/services/`; commands delegate to services.
- Comments and business-facing names in French.

### vue-frontend (`agent spec`)
- **No financial calculation on the frontend.** Display only what Rust returns (pre-formatted decimal strings).
- Follow the design system: `src/assets/styles/variables.css` — Rouge `#CC1A1A`, Bleu Nuit `#0A1628`, Or `#C4A152`, Bleu Royal `#1B3A6B`, Blanc Ivoire `#F7F5F0`. Fonts: Playfair Display (FR titles), Lato (FR body), Amiri (Arabic).
- RTL support for Arabic locale via vue-i18n.
- Composition API (`<script setup lang="ts">`) only — no Options API.
- One Pinia store per domain, zero business logic in stores/components.

## Key Business Concepts

### Quote lifecycle (statut)
`brouillon` → `finalise` → `envoye` → `accepte` / `perdu`

### Currency model
All quotes use `devise_achat` (default SAR) as the purchase currency. Three fixed exchange rates are stored per quote: `taux_sar_dzd`, `taux_usd_dzd`, `taux_eur_dzd`. All totals are in DZD. Margins can be `pourcentage` or `montant_fixe`.

### Passenger categories
`adulte`, `enfant_avec_lit`, `enfant_sans_lit`, `bebe` — each has distinct pricing.

### Catalogue entities
Hotels (`catalogue_hotels`) and airlines (`catalogue_compagnies`) provide pre-fill data for quote creation. CRUD on them is exposed via Tauri commands but they are not tied to a specific devis.

## Post-Review Status (after critical/important/minor fixes)

All Critical, Important, and Minor issues from the initial code review have been resolved:

### Compilation blockers fixed
- `commands/mod.rs` cleaned (orphans removed, all modules declared)
- `commands/devis.rs` fixed (missing brace, duplicate macro, garbled fragments)
- All Tauri commands in `main.rs` properly imported and exported
- CRUD implemented for previously empty files: `transferts.rs`, `prestations.rs`

### Schema alignment
- `database/schema.sql` aligned with all Rust model structs (passagers, segments_vol, hebergements, transferts, prestations_vip, clients, devis, parametres_agence, catalogue_hotels, catalogue_compagnies)
- Migration `0001_align_tables_models_rust.sql` applied via safe CREATE-new / INSERT / DROP-old / RENAME pattern

### Frontend completeness
- All 7 wizard components now fully implemented (was 2/7 stubs + `alert()` save)
- Missing TypeScript types added: `DevisCreate`, `DevisUpdate`, `ClientCreate`, `TotauxDevis`
- `MargeDashboardView` parseFloat violations replaced with string-formatter functions
- Full i18n FR/AR with section-level RTL (no global `document.dir`)

### Backend hardening
- PDF generation gated behind `pdf-generation` feature flag (`Cargo.toml`)
- `alerte_passeport` deduplicated: single source in `services/alerte_passeport.rs`, re-exported via `validation.rs`
- `utils/mod.rs` now re-exports `decimal_helpers`

### Verification
- `npm run build` passes 0 TypeScript errors
- Rust compilation requires `cargo check --all-targets` (PDF only with `--features pdf-generation`)

### Phase 1 & 2 Critical Fixes (July 2026)
- **C1 (rusqlite features)**: Added `chrono` and `rust_decimal` features to `rusqlite` in `Cargo.toml` to prevent compilation failure.
- **C2 (Date mapping)**: Fixed runtime database crashes by changing `NaiveDate` to `NaiveDateTime` for `created_at`, `updated_at`, and `date_creation` in Rust models.
- **C3 (unwrap panic)**: Replaced unsafe `.unwrap()` call on devis ID with safe `.ok_or()?` error handling in `commands/devis.rs`.
- **C4 (Financial inputs)**: Fixed data loss where exchange rates and margin settings were discarded by merging them in `handleUpdate` and loading them dynamically in `saveDevis`.
- **C5 (VIP Prestations)**: Pushed "Autre" custom prestations directly into `localPrestations` array instead of using separate refs, ensuring they are saved.
- **C6 (Haramain Train)**: Restored train transport data which was being discarded in `handleUpdate`.
- **C7 (Reactivity issues)**: Added deep watchers on `localSegments` and `localHebergements` in the wizard to ensure changes are emitted to parent on text edit.
- **C8 (Récapitulatif Totaux)**: Fixed blank/zero totals on the Recap step by passing the `totaux` prop to `RecapitulatifDevis`.

### Phase 3 Refactoring (July 2026)
- **Architecture Refactoring (T1 & T2)**: Moved all database queries (`rusqlite`) from command controllers (`src-tauri/src/commands/`) to a service layer (`src-tauri/src/services/db/`).
- **Global Error Handling**: Standardized error propagation using `AppError` and `AppResult` across all commands and database service files.
- **IPC controller cleanup**: Command files (`clients.rs`, `compagnies.rs`, `devis.rs`, `hebergements.rs`, `hotels.rs`, `passagers.rs`, `prestations.rs`, `transferts.rs`, `vols.rs`, `validation.rs`, `pdf.rs`, `marge.rs`) have been cleaned and now only delegate database tasks to `services/db/`.
- **Fixed decimals & types handling**: Handled decimal serialization as text strings inside database tables cleanly, using `Decimal::from_str` and correct rusqlite parameter passing (`params!`).


