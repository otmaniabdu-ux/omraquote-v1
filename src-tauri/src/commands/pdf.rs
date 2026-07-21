#[cfg(feature = "pdf-generation")]
mod pdf_impl {
    use crate::db::DbState;
    use crate::models::devis::Devis;
    use crate::services::generation_pdf::generer_pdf;
    use crate::services::conversion_devises::convertir_vers_dzd;
    use crate::services::calcul_prix::calculer_totaux_devis;
    use crate::services::calcul_nuitees::calculer_nuitees;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use serde_json::json;
    use tauri::State;
    use std::fs;
    use std::path::PathBuf;

    // Fonction pour construire les données JSON à injecter dans le template
    fn preparer_donnees_devis(
        devis: &Devis,
        client_nom: &str,
        passagers: &[crate::models::passager::Passager],
        segments: &[crate::models::segment_vol::SegmentVol],
        hebergements: &[crate::models::hebergement::Hebergement],
        transferts: &[crate::models::transfert::Transfert],
        prestations: &[crate::models::prestation_vip::PrestationVip],
        variante: &str, // "client" ou "interne"
    ) -> serde_json::Value {
        // Construire la liste des prestations (toutes les lignes)
        let mut prestations_list = Vec::new();

        // Vols
        for seg in segments {
            let total = seg.prix_adulte; // simplifié, on prend le prix adulte
            prestations_list.push(json!({
                "description_fr": format!("Vol {} {} - {}", seg.compagnie, seg.numero_vol.as_deref().unwrap_or(""), seg.date_vol),
                "description_ar": format!("رحلة {} {}", seg.compagnie, seg.date_vol), // à améliorer
                "quantite": 1,
                "prix_unitaire": seg.prix_adulte.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(), // pour interne
                "marge": "0".to_string(),
            }));
        }

        // Hébergements
        for heberg in hebergements {
            let nuitees = calculer_nuitees(heberg.date_checkin, heberg.date_checkout).unwrap_or(0);
            let total = heberg.prix_par_nuit * Decimal::from(nuitees);
            prestations_list.push(json!({
                "description_fr": format!("Hôtel {} - {} ({} nuits)", heberg.nom_hotel, heberg.ville, nuitees),
                "description_ar": format!("فندق {} - {} ({} ليالي)", heberg.nom_hotel, heberg.ville, nuitees),
                "quantite": nuitees,
                "prix_unitaire": heberg.prix_par_nuit.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(),
                "marge": "0".to_string(),
            }));
        }

        // Transferts
        for tr in transferts {
            let total = tr.prix_unitaire * Decimal::from(tr.nombre_vehicules);
            prestations_list.push(json!({
                "description_fr": format!("Transfert {} - {}", tr.trajet, tr.type_vehicule),
                "description_ar": format!("نقل {} - {}", tr.trajet, tr.type_vehicule),
                "quantite": tr.nombre_vehicules,
                "prix_unitaire": tr.prix_unitaire.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(),
                "marge": "0".to_string(),
            }));
        }

        // Prestations VIP
        for prest in prestations {
            let total = prest.prix_unitaire * Decimal::from(prest.quantite);
            prestations_list.push(json!({
                "description_fr": format!("{} - {}", prest.type_prestation, prest.description),
                "description_ar": format!("{} - {}", prest.type_prestation, prest.description), // à adapter
                "quantite": prest.quantite,
                "prix_unitaire": prest.prix_unitaire.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(),
                "marge": "0".to_string(),
            }));
        }

        // Calculer les totaux (on utilise le service)
        let (cout_net, marge, prix_vente) = calculer_totaux_devis(
            devis,
            segments,
            hebergements,
            transferts,
            prestations,
        ).unwrap_or((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO));

        // Pourcentage de marge (si marge_type est "pourcentage")
        let marge_pourcentage = if devis.marge_type == "pourcentage" {
            devis.marge_valeur
        } else {
            Decimal::ZERO
        };

        json!({
            "numero_devis": devis.numero_devis,
            "date_depart": devis.date_depart.to_string(),
            "date_retour": devis.date_retour.to_string(),
            "date_depart_ar": devis.date_depart.to_string(), // format arabe à personnaliser
            "date_retour_ar": devis.date_retour.to_string(),
            "type_visa": devis.type_visa,
            "type_visa_ar": match devis.type_visa.as_str() {
                "omra_standard" => "عمرة قياسية",
                "touristique" => "سياحية",
                "hadj" => "حج",
                _ => "غير محدد",
            },
            "client_nom": client_nom,
            "client_nom_ar": client_nom, // à remplacer par un nom arabe si disponible
            "prestations": prestations_list,
            "cout_net_total": cout_net.to_string(),
            "montant_marge": marge.to_string(),
            "marge_pourcentage": marge_pourcentage.to_string(),
            "prix_vente_total": prix_vente.to_string(),
            "agence_nom_fr": "El Mouhssinouen Tours",
            "agence_nom_ar": "المحسنون للسياحة",
            "agence_adresse": "Algérie",
            "agence_telephone": "+213 XX XX XX XX",
            "agence_email": "contact@elmouhssinouen.dz",
            "agence_agrement": "12345",
        })
    }

    #[tauri::command]
    pub fn generate_pdf_client(
        state: State<DbState>,
        devis_id: i64,
        output_path: String,
    ) -> Result<(), String> {
        generate_pdf(state, devis_id, output_path, "client")
    }

    #[tauri::command]
    pub fn generate_pdf_interne(
        state: State<DbState>,
        devis_id: i64,
        output_path: String,
    ) -> Result<(), String> {
        generate_pdf(state, devis_id, output_path, "interne")
    }

    fn generate_pdf(
        state: State<DbState>,
        devis_id: i64,
        output_path: String,
        variante: &str,
    ) -> Result<(), String> {
        let conn = state.0.lock().map_err(|e| e.to_string())?;

        // 1. Récupérer le devis
        let devis = crate::commands::devis::get_devis_by_id(state.clone(), devis_id)?;

        // 2. Récupérer le client
        let client = crate::commands::clients::get_client_by_id(state.clone(), devis.client_id)?;
        let client_nom = client.nom_contact.as_deref().unwrap_or("Client");

        // 3. Récupérer les passagers (pour le détail éventuel)
        let mut stmt = conn.prepare(
            "SELECT * FROM passagers WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
        let mut passagers = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            // Récupérer les champs (similaire à list_passagers_by_devis)
            // On va simplifier en appelant la commande existante
            passagers = crate::commands::passagers::list_passagers_by_devis(state.clone(), devis_id)?;
        }

        // 4. Récupérer les segments de vol
        let mut stmt = conn.prepare(
            "SELECT * FROM segments_vol WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
        let mut segments = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            // Construction manuelle pour l'exemple
            // (On pourrait utiliser une fonction utilitaire)
            segments.push(crate::models::segment_vol::SegmentVol {
                id: row.get(0)?,
                devis_id: row.get(1)?,
                ordre: row.get(2)?,
                compagnie: row.get(3)?,
                numero_vol: row.get(4)?,
                classe: row.get(5)?,
                date_vol: row.get(6)?,
                aeroport_depart: row.get(7)?,
                aeroport_arrivee: row.get(8)?,
                heure_depart: row.get(9)?,
                heure_arrivee: row.get(10)?,
                prix_adulte: row.get(11)?,
                prix_enfant: row.get(12)?,
                prix_bebe: row.get(13)?,
                devise_prix: row.get(14)?,
                remarques: row.get(15)?,
            });
        }

        // 5. Récupérer les hébergements
        let mut stmt = conn.prepare(
            "SELECT * FROM hebergements WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
        let mut hebergements = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            hebergements.push(crate::models::hebergement::Hebergement {
                id: row.get(0)?,
                devis_id: row.get(1)?,
                ville: row.get(2)?,
                nom_hotel: row.get(3)?,
                type_chambre: row.get(4)?,
                formule_repas: row.get(5)?,
                vue: row.get(6)?,
                date_checkin: row.get(7)?,
                date_checkout: row.get(8)?,
                nb_nuitees: row.get(9)?,
                prix_par_nuit: row.get(10)?,
                devise_prix: row.get(11)?,
                taxes_incluses: row.get(12)?,
                remarques: row.get(13)?,
            });
        }

        // 6. Transferts
        let mut stmt = conn.prepare(
            "SELECT * FROM transferts WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
        let mut transferts = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            transferts.push(crate::models::transfert::Transfert {
                id: row.get(0)?,
                devis_id: row.get(1)?,
                type_transfert: row.get(2)?,
                trajet: row.get(3)?,
                type_vehicule: row.get(4)?,
                date_transfert: row.get(5)?,
                heure_transfert: row.get(6)?,
                nombre_vehicules: row.get(7)?,
                prix_unitaire: row.get(8)?,
                devise_prix: row.get(9)?,
                remarques: row.get(10)?,
            });
        }

        // 7. Prestations VIP
        let mut stmt = conn.prepare(
            "SELECT * FROM prestations_vip WHERE devis_id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query(&[&devis_id]).map_err(|e| e.to_string())?;
        let mut prestations_vip = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            prestations_vip.push(crate::models::prestation_vip::PrestationVip {
                id: row.get(0)?,
                devis_id: row.get(1)?,
                type_prestation: row.get(2)?,
                description: row.get(3)?,
                prix_unitaire: row.get(4)?,
                quantite: row.get(5)?,
                devise_prix: row.get(6)?,
                remarques: row.get(7)?,
            });
        }

        // 8. Construire les données JSON
        let donnees = preparer_donnees_devis(
            &devis,
            client_nom,
            &passagers,
            &segments,
            &hebergements,
            &transferts,
            &prestations_vip,
            variante,
        );

        // 9. Lire le template Typst
        let template_str = include_str!("../../../../templates/devis_pdf.typ")
            .to_string();

        // 10. Charger les polices
        let polices = charger_polices();

        // 11. Générer le PDF
        let pdf_bytes = generer_pdf(&template_str, donnees, polices)
            .map_err(|e| format!("Erreur de génération PDF: {}", e))?;

        // 12. Écrire le fichier
        let path = PathBuf::from(&output_path);
        fs::write(&path, pdf_bytes)
            .map_err(|e| format!("Erreur d'écriture du PDF: {}", e))?;

        Ok(())
    }

    // Charger les polices embarquées dans le binaire
    fn charger_polices() -> Vec<(&'static [u8], &'static str)> {
        vec![
            (include_bytes!("../../../../assets/fonts/PlayfairDisplay-Regular.ttf").as_slice(), "Playfair Display"),
            (include_bytes!("../../../../assets/fonts/PlayfairDisplay-Bold.ttf").as_slice(), "Playfair Display Bold"),
            (include_bytes!("../../../../assets/fonts/Lato-Regular.ttf").as_slice(), "Lato"),
        ]
    }
}

// --- Implémentations de repli (feature pdf-generation désactivée) ---
#[cfg(not(feature = "pdf-generation"))]
mod pdf_fallback {
    use tauri::State;
    use crate::db::DbState;

    /// Fonction de repli lorsque le module PDF n'est pas compilé.
    #[tauri::command]
    pub fn generate_pdf_client(
        _state: State<DbState>,
        _devis_id: i64,
        _output_path: String,
    ) -> Result<(), String> {
        Err("Génération PDF désactivée : recompiler avec --features pdf-generation".to_string())
    }

    /// Fonction de repli lorsque le module PDF n'est pas compilé.
    #[tauri::command]
    pub fn generate_pdf_interne(
        _state: State<DbState>,
        _devis_id: i64,
        _output_path: String,
    ) -> Result<(), String> {
        Err("Génération PDF désactivée : recompiler avec --features pdf-generation".to_string())
    }
}

// Ré-exporter les commandes au niveau module
#[cfg(feature = "pdf-generation")]
pub use pdf_impl::{generate_pdf_client, generate_pdf_interne};

#[cfg(not(feature = "pdf-generation"))]
pub use pdf_fallback::{generate_pdf_client, generate_pdf_interne};
