-- ============================================================
-- SCHEMA SQLite — OmraVIP Quotes
-- El Mouhssinouen Tours (المحسنون للسياحة)
-- ------------------------------------------------------------
-- RÈGLE ABSOLUE : toute colonne monétaire est stockée en TEXT
-- (chaîne décimale, ex: "125430.50"). JAMAIS de REAL/FLOAT.
-- Les calculs se font côté Rust avec rust_decimal::Decimal.
-- SQLite n'a pas de type DECIMAL natif : TEXT est le choix
-- correct pour préserver l'exactitude.
-- ============================================================

PRAGMA foreign_keys = ON;

-- ------------------------------------------------------------
-- 1. PARAMÈTRES AGENCE (ligne unique, id=1)
-- ------------------------------------------------------------
CREATE TABLE parametres_agence (
    id                      INTEGER PRIMARY KEY CHECK (id = 1),
    nom_agence_fr           TEXT NOT NULL DEFAULT 'El Mouhssinouen Tours',
    nom_agence_ar           TEXT NOT NULL DEFAULT 'المحسنون للسياحة',
    adresse                 TEXT,
    telephone               TEXT,
    email                   TEXT,
    site_web                TEXT,
    numero_agrement         TEXT,
    logo_path               TEXT,
    devise_defaut           TEXT NOT NULL DEFAULT 'DZD'
                                CHECK(devise_defaut IN ('DZD','SAR','USD','EUR')),
    taux_tva                TEXT,
    -- Champs legacy (compatibilité migration 0001)
    marge_defaut_type       TEXT NOT NULL DEFAULT 'pourcentage'
                                CHECK(marge_defaut_type IN ('pourcentage','montant_fixe')),
    marge_defaut_valeur     TEXT NOT NULL DEFAULT '15',
    devise_reference        TEXT NOT NULL DEFAULT 'DZD',
    seuil_alerte_passeport_mois INTEGER NOT NULL DEFAULT 6,
    derniere_maj            TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO parametres_agence (id) VALUES (1);

-- ------------------------------------------------------------
-- 2. CLIENTS
-- ------------------------------------------------------------
CREATE TABLE clients (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    code_client     TEXT NOT NULL UNIQUE,
    raison_sociale  TEXT,
    nom_contact     TEXT,
    telephone       TEXT,
    email           TEXT,
    adresse         TEXT,
    pays            TEXT,
    type_client     TEXT NOT NULL DEFAULT 'particulier'
                                CHECK(type_client IN ('particulier','agence')),
    remarques       TEXT,
    created_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_clients_code ON clients(code_client);

-- ------------------------------------------------------------
-- 3. DEVIS (dossier maître)
-- ------------------------------------------------------------
CREATE TABLE devis (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    numero_devis            TEXT NOT NULL UNIQUE,   -- format DEVIS-YYYY-MM-NNN
    client_id               INTEGER NOT NULL REFERENCES clients(id) ON DELETE RESTRICT,

    type_visa               TEXT NOT NULL CHECK(type_visa IN ('omra_standard','touristique','hadj')),

    date_creation           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_depart             TEXT NOT NULL,
    date_retour             TEXT NOT NULL,
    statut                  TEXT NOT NULL DEFAULT 'brouillon'
                                CHECK(statut IN ('brouillon','finalise','envoye','accepte','perdu')),

    -- Assurance & devise achat
    assurance_medicale      BOOLEAN NOT NULL DEFAULT 0,
    devise_achat            TEXT CHECK(devise_achat IN ('DZD','SAR','USD','EUR')),

    -- Taux de change VERROUILLÉS à la création du devis (chaîne décimale)
    -- Valeur = combien de DZD pour 1 unité de la devise
    taux_sar_dzd            TEXT NOT NULL,
    taux_usd_dzd            TEXT NOT NULL,
    taux_eur_dzd            TEXT NOT NULL,

    -- Marge agence
    marge_type              TEXT NOT NULL CHECK(marge_type IN ('pourcentage','montant_fixe')),
    marge_valeur            TEXT NOT NULL,

    -- Totaux calculés et mis en cache (en DZD, chaîne décimale)
    -- Recalculés par le backend à chaque modification d'une ligne du devis
    cout_net_total          TEXT NOT NULL DEFAULT '0',
    marge_montant_total     TEXT NOT NULL DEFAULT '0',
    prix_vente_total        TEXT NOT NULL DEFAULT '0',

    -- Remise et notes
    remise                  TEXT DEFAULT '0',
    notes_internes          TEXT,
    updated_at              TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_devis_client   ON devis(client_id);
CREATE INDEX idx_devis_numero   ON devis(numero_devis);
CREATE INDEX idx_devis_statut   ON devis(statut);

-- ------------------------------------------------------------
-- 4. PASSAGERS (rattachés à un devis)
-- -----------------------------------------------------------
CREATE TABLE passagers (
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id                    INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    categorie                   TEXT NOT NULL CHECK(categorie IN ('adulte','enfant_avec_lit','enfant_sans_lit','bebe')),
    nom_complet                 TEXT NOT NULL,
    date_naissance              TEXT,

    -- Sécurité passeport
    nationalite                 TEXT,
    numero_passeport            TEXT,
    date_expiration_passeport   TEXT,
    lieu_delivrance             TEXT,

    -- Champs futurs (garde-robe pour évolutions)
    alerte_passeport            INTEGER NOT NULL DEFAULT 0,
    type_visa                   TEXT CHECK(type_visa IN ('omra_standard','touristique','hadj')),
    cout_assurance              TEXT NOT NULL DEFAULT '0',   -- décimal string
    devise_assurance            TEXT NOT NULL DEFAULT 'SAR' CHECK(devise_assurance IN ('DZD','SAR','USD','EUR')),

    remarques                   TEXT
);

CREATE INDEX idx_passagers_devis ON passagers(devis_id);

-- ------------------------------------------------------------
-- 5. SEGMENTS DE VOL (multi-destinations, ex: ALG->MED / JED->ALG)
-- ------------------------------------------------------------
CREATE TABLE segments_vol (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    ordre           INTEGER NOT NULL,           -- ordre d'affichage du segment
    origine         TEXT NOT NULL,
    destination     TEXT NOT NULL,
    compagnie       TEXT NOT NULL,
    classe          TEXT NOT NULL CHECK(classe IN ('economique','affaires','premiere')),
    date_vol        TEXT NOT NULL,

    -- Informations vol
    numero_vol      TEXT,
    aeroport_depart TEXT,
    aeroport_arrivee TEXT,
    heure_depart    TEXT,
    heure_arrivee   TEXT,

    -- Tarification par tranche d'âge
    prix_adulte     TEXT NOT NULL,
    prix_enfant     TEXT NOT NULL,
    prix_bebe       TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    remarques       TEXT
);

CREATE INDEX idx_segments_devis ON segments_vol(devis_id);

-- ------------------------------------------------------------
-- 6. HÉBERGEMENTS (Makkah & Médine)
-- ------------------------------------------------------------
CREATE TABLE hebergements (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    ville           TEXT NOT NULL CHECK(ville IN ('Makkah','Medine')),
    nom_hotel       TEXT NOT NULL,
    type_chambre    TEXT NOT NULL CHECK(type_chambre IN ('single','double','triple','quadruple')),
    formule_repas   TEXT CHECK(formule_repas IN ('petit_dejeuner','demi_pension','pension_complete')),
    vue             TEXT CHECK(vue IN ('Kaaba','Haram','City')),

    date_checkin    TEXT NOT NULL,
    date_checkout   TEXT NOT NULL,
    nb_nuitees      INTEGER,           -- CALCULÉ par le backend (chrono, jamais julianday() flottant)

    prix_par_nuit   TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    taxes_incluses  BOOLEAN NOT NULL DEFAULT 0,
    remarques       TEXT
);

CREATE INDEX idx_heberg_devis ON hebergements(devis_id);

-- ------------------------------------------------------------
-- 7. TRANSFERTS TERRESTRES VIP
-- ------------------------------------------------------------
CREATE TABLE transferts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    type_transfert  TEXT NOT NULL CHECK(type_transfert IN ('obligatoire','optionnel')),
    trajet          TEXT NOT NULL,          -- ex: 'Aéroport Jeddah -> Hôtel Makkah'
    type_vehicule   TEXT NOT NULL CHECK(type_vehicule IN ('GMC_Yukon','Mercedes_Classe_E','Bus_VIP_prive')),

    date_transfert  TEXT,
    heure_transfert TEXT,
    nombre_vehicules INTEGER NOT NULL DEFAULT 1,

    prix_unitaire   TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    remarques       TEXT
);

CREATE INDEX idx_transferts_devis ON transferts(devis_id);

-- ------------------------------------------------------------
-- 8. TRAIN HARAMAIN
-- ------------------------------------------------------------
CREATE TABLE train_haramain (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    trajet          TEXT NOT NULL,          -- ex: 'Makkah-Medine'
    classe          TEXT NOT NULL CHECK(classe IN ('economique','business')),
    prix            TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR'))
);

CREATE INDEX idx_train_devis ON train_haramain(devis_id);

-- ------------------------------------------------------------
-- 9. PRESTATIONS VIP EXCLUSIVES
-- ------------------------------------------------------------
CREATE TABLE prestations_vip (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    type_prestation TEXT NOT NULL CHECK(type_prestation IN ('ziyarat','lounge','fast_track','bagages','zamzam','autre')),
    description     TEXT NOT NULL,
    prix_unitaire   TEXT NOT NULL,
    quantite        INTEGER NOT NULL DEFAULT 1,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),
    remarques       TEXT
);

CREATE INDEX idx_prestations_devis ON prestations_vip(devis_id);

-- ------------------------------------------------------------
-- 10. CATALOGUES RÉUTILISABLES (pour accélérer la saisie)
-- ------------------------------------------------------------
CREATE TABLE catalogue_hotels (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    nom_hotel       TEXT NOT NULL,
    ville           TEXT CHECK(ville IN ('Makkah','Medine')),
    categorie       TEXT,
    adresse         TEXT,
    contact_fournisseur TEXT,
    site_web        TEXT,
    remarques       TEXT,
    actif           BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE catalogue_compagnies (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    code_iata   TEXT,
    nom_compagnie TEXT NOT NULL,
    pays        TEXT,
    site_web    TEXT,
    contact     TEXT,
    notes       TEXT,
    actif       BOOLEAN NOT NULL DEFAULT 1
);

-- ------------------------------------------------------------
-- 11. NUMÉROTATION SÉQUENTIELLE (DEVIS-YYYY-MM-NNN)
-- ------------------------------------------------------------
CREATE TABLE compteurs_numerotation (
    cle             TEXT PRIMARY KEY,      -- ex: 'DEVIS-2026-07'
    dernier_numero  INTEGER NOT NULL DEFAULT 0
);
