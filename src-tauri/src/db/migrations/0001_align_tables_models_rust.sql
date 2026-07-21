-- ============================================================
-- MIGRATION 0001 — Alignement schema.sql <-> modÃ¨les Rust
-- Date:   2026-07-21
-- Source: Revue cross-reference de src-tauri/src/models/*.rs
--         vs database/schema.sql
-- ============================================================

PRAGMA foreign_keys = OFF;

BEGIN TRANSACTION;

-- ------------------------------------------------------------
-- TABLE 1 : clients (renommÃ©e et restructurÃ©e)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS clients_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    code_client     TEXT NOT NULL UNIQUE,
    raison_sociale  TEXT,
    nom_contact     TEXT,
    telephone       TEXT,
    email           TEXT,
    adresse         TEXT,
    pays            TEXT,
    type_client     TEXT NOT NULL DEFAULT 'particulier'
                        CHECK (type_client IN ('particulier','agence')),
    remarques       TEXT,
    created_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO clients_new
SELECT id, NULL, NULL, telephone, email, adresse, wilaya, 'particulier', notes, date_creation, date_creation
FROM clients;

DROP TABLE clients;
ALTER TABLE clients_new RENAME TO clients;

CREATE INDEX idx_clients_code ON clients(code_client);

-- ------------------------------------------------------------
-- TABLE 2 : passagers (colonnes nom/civilite fusionnÃ©es en nom_complet)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS passagers_new (
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id                    INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    categorie                   TEXT NOT NULL CHECK(categorie IN ('adulte','enfant_avec_lit','enfant_sans_lit','bebe')),
    nom_complet                 TEXT NOT NULL,
    date_naissance              TEXT,

    -- Passeport
    nationalite                 TEXT,
    numero_passeport            TEXT,
    date_expiration_passeport   TEXT,
    lieu_delivrance             TEXT,

    -- Champs futurs (garde-robe pour Ã©volsions)
    alerte_passeport            INTEGER NOT NULL DEFAULT 0,
    type_visa                   TEXT CHECK(type_visa IN ('omra_standard','touristique','hadj')),
    cout_assurance              TEXT NOT NULL DEFAULT '0',
    devise_assurance              TEXT NOT NULL DEFAULT 'SAR' CHECK(devise_assurance IN ('DZD','SAR','USD','EUR')),

    remarques                   TEXT
);

INSERT INTO passagers_new
SELECT id, devis_id, categorie, nom || ' ' || prenom, date_naissance,
       NULL, numero_passeport, date_expiration_passeport, NULL,
       alerte_passeport, type_visa, cout_assurance, devise_assurance, remarques
FROM passagers;

DROP TABLE passagers;
ALTER TABLE passagers_new RENAME TO passagers;

CREATE INDEX idx_passagers_devis ON passagers(devis_id);

-- ------------------------------------------------------------
-- TABLE 3 : segments_vol (colonnes vol + CHECK classe corrigÃ©)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS segments_vol_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    ordre           INTEGER NOT NULL,
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

    -- Tarification par tranche d'Ã¢ge
    prix_adulte     TEXT NOT NULL,
    prix_enfant     TEXT NOT NULL,
    prix_bebe       TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    remarques       TEXT
);

INSERT INTO segments_vol_new
SELECT id, devis_id, ordre, origine, destination, compagnie, classe, date_vol,
       NULL, NULL, NULL, NULL, NULL,
       prix_adulte, prix_enfant, prix_bebe, devise, NULL
FROM segments_vol;

DROP TABLE segments_vol;
ALTER TABLE segments_vol_new RENAME TO segments_vol;

CREATE INDEX idx_segments_devis ON segments_vol(devis_id);

-- ------------------------------------------------------------
-- TABLE 4 : hebergements (renommage colonnes + ajouts)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS hebergements_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    ville           TEXT NOT NULL CHECK(ville IN ('Makkah','Medine')),
    nom_hotel       TEXT NOT NULL,
    type_chambre    TEXT NOT NULL CHECK(type_chambre IN ('single','double','triple','quadruple')),
    formule_repas   TEXT CHECK(formule_repas IN ('petit_dejeuner','demi_pension','pension_complete')),
    vue             TEXT CHECK(vue IN ('Kaaba','Haram','City')),

    date_checkin    TEXT NOT NULL,
    date_checkout   TEXT NOT NULL,
    nb_nuitees      INTEGER,

    prix_par_nuit   TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    taxes_incluses  BOOLEAN NOT NULL DEFAULT 0,
    remarques       TEXT
);

INSERT INTO hebergements_new
SELECT id, devis_id, ville, nom_hotel, type_chambre, formule_repas, vue,
       date_checkin, date_checkout, NULL, prix_nuit, devise, 0, NULL
FROM hebergements;

DROP TABLE hebergements;
ALTER TABLE hebergements_new RENAME TO hebergements;

CREATE INDEX idx_heberg_devis ON hebergements(devis_id);

-- ------------------------------------------------------------
-- TABLE 5 : transferts (ajouts de colonnes)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS transferts_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    type_transfert  TEXT NOT NULL CHECK(type_transfert IN ('obligatoire','optionnel')),
    trajet          TEXT NOT NULL,
    type_vehicule   TEXT NOT NULL CHECK(type_vehicule IN ('GMC_Yukon','Mercedes_Classe_E','Bus_VIP_prive')),

    date_transfert  TEXT,
    heure_transfert TEXT,
    nombre_vehicules INTEGER NOT NULL DEFAULT 1,

    prix_unitaire   TEXT NOT NULL,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),

    remarques       TEXT
);

INSERT INTO transferts_new
SELECT id, devis_id, type_transfert, trajet, type_vehicule,
       NULL, NULL, 1, prix, devise, NULL
FROM transferts;

DROP TABLE transferts;
ALTER TABLE transferts_new RENAME TO transferts;

CREATE INDEX idx_transferts_devis ON transferts(devis_id);

-- ------------------------------------------------------------
-- TABLE 6 : prestations_vip (ajouts de colonnes)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS prestations_vip_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    devis_id        INTEGER NOT NULL REFERENCES devis(id) ON DELETE CASCADE,
    type_prestation TEXT NOT NULL CHECK(type_prestation IN ('ziyarat','lounge','fast_track','bagages','zamzam','autre')),
    description     TEXT NOT NULL,
    prix_unitaire   TEXT NOT NULL,
    quantite        INTEGER NOT NULL DEFAULT 1,
    devise_prix     TEXT NOT NULL CHECK(devise_prix IN ('DZD','SAR','USD','EUR')),
    remarques       TEXT
);

INSERT INTO prestations_vip_new
SELECT id, devis_id, type_prestation, COALESCE(description, ''), prix, 1, devise, NULL
FROM prestations_vip;

DROP TABLE prestations_vip;
ALTER TABLE prestations_vip_new RENAME TO prestations_vip;

CREATE INDEX idx_prestations_devis ON prestations_vip(devis_id);

-- ------------------------------------------------------------
-- TABLE 7 : devis (renommage + ajouts)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS devis_new (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    numero_devis            TEXT NOT NULL UNIQUE,
    client_id               INTEGER NOT NULL REFERENCES clients(id) ON DELETE RESTRICT,

    -- Type visa (Ã©tait type_pelerinage dans ancian schema)
    type_visa               TEXT NOT NULL CHECK(type_visa IN ('omra_standard','touristique','hadj')),

    date_creation           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_depart             TEXT NOT NULL,
    date_retour             TEXT NOT NULL,
    statut                  TEXT NOT NULL DEFAULT 'brouillon'
                                CHECK(statut IN ('brouillon','finalise','envoye','accepte','perdu')),

    -- Assurance & devise achat
    assurance_medicale      BOOLEAN NOT NULL DEFAULT 0,
    devise_achat            TEXT CHECK(devise_achat IN ('DZD','SAR','USD','EUR')),

    -- Taux de change VERROUILLÃ©s Ã la crÃ©ation (chaÃ®ne dÃ©cimale)
    taux_sar_dzd            TEXT NOT NULL,
    taux_usd_dzd            TEXT NOT NULL,
    taux_eur_dzd            TEXT NOT NULL,

    -- Marge agence
    marge_type              TEXT NOT NULL CHECK(marge_type IN ('pourcentage','montant_fixe')),
    marge_valeur            TEXT NOT NULL,

    -- Totaux (TEXT dÃ©cimal, pas de vues)
    cout_net_total          TEXT NOT NULL DEFAULT '0',
    marge_montant_total     TEXT NOT NULL DEFAULT '0',
    prix_vente_total        TEXT NOT NULL DEFAULT '0',

    -- Remise et notes
    remise                  TEXT DEFAULT '0',
    notes_internes          TEXT,
    updated_at              TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO devis_new
SELECT id, numero_devis, client_id,
       type_pelerinage, date_creation, date_depart, date_retour, statut,
       0, NULL, taux_sar_dzd, taux_usd_dzd, taux_eur_dzd,
       marge_type, marge_valeur, cout_net_total, marge_montant_total, prix_vente_total,
       '0', notes_internes, updated_at
FROM devis;

DROP TABLE devis;
ALTER TABLE devis_new RENAME TO devis;

CREATE INDEX idx_devis_client   ON devis(client_id);
CREATE INDEX idx_devis_numero   ON devis(numero_devis);
CREATE INDEX idx_devis_statut   ON devis(statut);

-- ------------------------------------------------------------
-- TABLE 8 : parametres_agence (renommage + ajouts)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS parametres_agence_new (
    id                      INTEGER PRIMARY KEY CHECK(id = 1),
    nom_agence_fr           TEXT NOT NULL DEFAULT 'El Mouhssinouen Tours',
    nom_agence_ar           TEXT NOT NULL DEFAULT 'المحسنون للسياحة',
    adresse                 TEXT,
    telephone               TEXT,
    email                   TEXT,
    site_web                TEXT,
    numero_agrement         TEXT,
    logo_path               TEXT,
    devise_defaut           TEXT NOT NULL DEFAULT 'DZD' CHECK(devise_defaut IN ('DZD','SAR','USD','EUR')),
    taux_tva                TEXT,

    -- Anciens champs gardÃ©s pour compatibilitÃ© temporaire
    marge_defaut_type       TEXT NOT NULL DEFAULT 'pourcentage'
                                CHECK(marge_defaut_type IN ('pourcentage','montant_fixe')),
    marge_defaut_valeur     TEXT NOT NULL DEFAULT '15',
    devise_reference        TEXT NOT NULL DEFAULT 'DZD',
    seuil_alerte_passeport_mois INTEGER NOT NULL DEFAULT 6,
    derniere_maj            TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO parametres_agence_new
SELECT id, nom_agence, nom_agence_ar, adresse, telephone, email, NULL, NULL, logo_path, devise_reference, NULL,
       marge_defaut_type, marge_defaut_valeur, devise_reference, seuil_alerte_passeport_mois, derniere_maj
FROM parametres_agence;

DROP TABLE parametres_agence;
ALTER TABLE parametres_agence_new RENAME TO parametres_agence;

-- ------------------------------------------------------------
-- TABLE 9 : catalogue_hotels (renommage + ajouts)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS catalogue_hotels_new (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    nom_hotel               TEXT NOT NULL,
    ville                   TEXT CHECK(ville IN ('Makkah','Medine')),
    categorie               TEXT,
    adresse                 TEXT,
    contact_fournisseur     TEXT,
    site_web                TEXT,
    remarques               TEXT,
    actif                   BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO catalogue_hotels_new
SELECT id, nom_hotel, ville,
       CASE WHEN categorie_etoiles IS NOT NULL THEN CAST(categorie_etoiles || '_etoiles' AS TEXT) ELSE NULL END,
       NULL, contact_fournisseur, NULL, notes, 1
FROM catalogue_hotels;

DROP TABLE catalogue_hotels;
ALTER TABLE catalogue_hotels_new RENAME TO catalogue_hotels;

-- ------------------------------------------------------------
-- TABLE 10 : catalogue_compagnies (ajouts)
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS catalogue_compagnies_new (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    code_iata       TEXT,
    nom_compagnie   TEXT NOT NULL,
    pays            TEXT,
    site_web        TEXT,
    contact         TEXT,
    notes           TEXT,
    actif           BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO catalogue_compagnies_new
SELECT id, NULL, nom_compagnie, NULL, NULL, contact, notes, 1
FROM catalogue_compagnies;

DROP TABLE catalogue_compagnies;
ALTER TABLE catalogue_compagnies_new RENAME TO catalogue_compagnies;

-- ------------------------------------------------------------
-- TRAIN_HARAMAIN : aucune modification requise (compatible)
-- COMPTEURS_NUMEROTATION : aucune modification requise
-- ------------------------------------------------------------

COMMIT;

PRAGMA foreign_keys = ON;
