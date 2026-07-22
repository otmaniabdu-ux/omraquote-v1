use crate::db::DbState;
use crate::error::{AppError, AppResult};
use crate::models::devis::Devis;
use tauri::State;

#[cfg(feature = "pdf-generation")]
mod pdf_impl {
    use super::*;
    use crate::services::generation_pdf::generer_pdf;
    use crate::services::calcul_prix::calculer_totaux_devis;
    use crate::services::calcul_nuitees::calculer_nuitees;
    use crate::services::db::{clients, devis, passagers, vols, hebergements, transferts, prestations};
    use rust_decimal::Decimal;
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;

    fn preparer_donnees_devis(
        devis_obj: &Devis,
        client_nom: &str,
        passagers_list: &[crate::models::passager::Passager],
        segments: &[crate::models::segment_vol::SegmentVol],
        hebergements_list: &[crate::models::hebergement::Hebergement],
        transferts_list: &[crate::models::transfert::Transfert],
        prestations_list_objs: &[crate::models::prestation_vip::PrestationVip],
        _variante: &str,
    ) -> serde_json::Value {
        let mut prestations_list = Vec::new();

        // Vols
        for seg in segments {
            let total = seg.prix_adulte;
            prestations_list.push(json!({
                "description_fr": format!("Vol {} {} - {}", seg.compagnie, seg.numero_vol.as_deref().unwrap_or(""), seg.date_vol),
                "description_ar": format!("رحلة {} {}", seg.compagnie, seg.date_vol),
                "quantite": 1,
                "prix_unitaire": seg.prix_adulte.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(),
                "marge": "0".to_string(),
            }));
        }

        // Hébergements
        for heberg in hebergements_list {
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
        for tr in transferts_list {
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
        for prest in prestations_list_objs {
            let total = prest.prix_unitaire * Decimal::from(prest.quantite);
            prestations_list.push(json!({
                "description_fr": format!("{} - {}", prest.type_prestation, prest.description),
                "description_ar": format!("{} - {}", prest.type_prestation, prest.description),
                "quantite": prest.quantite,
                "prix_unitaire": prest.prix_unitaire.to_string(),
                "total": total.to_string(),
                "cout_net": total.to_string(),
                "marge": "0".to_string(),
            }));
        }

        let (cout_net, marge, prix_vente) = calculer_totaux_devis(
            devis_obj,
            segments,
            hebergements_list,
            transferts_list,
            prestations_list_objs,
        ).unwrap_or((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO));

        let marge_pourcentage = if devis_obj.marge_type == "pourcentage" {
            devis_obj.marge_valeur
        } else {
            Decimal::ZERO
        };

        json!({
            "numero_devis": devis_obj.numero_devis,
            "date_depart": devis_obj.date_depart.to_string(),
            "date_retour": devis_obj.date_retour.to_string(),
            "date_depart_ar": devis_obj.date_depart.to_string(),
            "date_retour_ar": devis_obj.date_retour.to_string(),
            "type_visa": devis_obj.type_visa,
            "type_visa_ar": match devis_obj.type_visa.as_str() {
                "omra_standard" => "عمرة قياسية",
                "touristique" => "سياحية",
                "hadj" => "حج",
                _ => "غير محدد",
            },
            "client_nom": client_nom,
            "client_nom_ar": client_nom,
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
    ) -> AppResult<()> {
        generate_pdf(state, devis_id, output_path, "client")
    }

    #[tauri::command]
    pub fn generate_pdf_interne(
        state: State<DbState>,
        devis_id: i64,
        output_path: String,
    ) -> AppResult<()> {
        generate_pdf(state, devis_id, output_path, "interne")
    }

    fn generate_pdf(
        state: State<DbState>,
        devis_id: i64,
        output_path: String,
        variante: &str,
    ) -> AppResult<()> {
        let conn = state.0.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        // 1. Récupérer le devis
        let devis_obj = devis::get_by_id(&conn, devis_id)?;

        // 2. Récupérer le client
        let client = clients::get_by_id(&conn, devis_obj.client_id)?;
        let client_nom = client.nom_contact.as_deref().unwrap_or("Client");

        // 3. Récupérer les passagers
        let passagers_list = passagers::list_by_devis(&conn, devis_id)?;

        // 4. Récupérer les segments de vol
        let segments = vols::list_by_devis(&conn, devis_id)?;

        // 5. Récupérer les hébergements
        let hebergements_list = hebergements::list_hebergements_by_devis(&conn, devis_id)?;

        // 6. Transferts
        let transferts_list = transferts::list_by_devis(&conn, devis_id)?;

        // 7. Prestations VIP
        let prestations_vip = prestations::list_by_devis(&conn, devis_id)?;

        // 8. Construire les données JSON
        let donnees = preparer_donnees_devis(
            &devis_obj,
            client_nom,
            &passagers_list,
            &segments,
            &hebergements_list,
            &transferts_list,
            &prestations_vip,
            variante,
        );

        // 9. Lire le template Typst
        let template_str = include_str!("../../../../templates/devis_pdf.typ").to_string();

        // 10. Charger les polices
        let polices = charger_polices();

        // 11. Générer le PDF
        let pdf_bytes = generer_pdf(&template_str, donnees, polices)
            .map_err(|e| AppError::Internal(format!("Erreur de génération PDF: {}", e)))?;

        // 12. Écrire le fichier
        let path = PathBuf::from(&output_path);
        fs::write(&path, pdf_bytes)
            .map_err(|e| AppError::Internal(format!("Erreur d'écriture du PDF: {}", e)))?;

        Ok(())
    }

    fn charger_polices() -> Vec<(&'static [u8], &'static str)> {
        vec![
            (include_bytes!("../../../../assets/fonts/PlayfairDisplay-Regular.ttf").as_slice(), "Playfair Display"),
            (include_bytes!("../../../../assets/fonts/PlayfairDisplay-Bold.ttf").as_slice(), "Playfair Display Bold"),
            (include_bytes!("../../../../assets/fonts/Lato-Regular.ttf").as_slice(), "Lato"),
        ]
    }
}

#[cfg(not(feature = "pdf-generation"))]
mod pdf_fallback {
    use super::*;

    #[tauri::command]
    pub fn generate_pdf_client(
        _state: State<DbState>,
        _devis_id: i64,
        _output_path: String,
    ) -> AppResult<()> {
        Err(AppError::Internal("Génération PDF désactivée : recompiler avec --features pdf-generation".to_string()))
    }

    #[tauri::command]
    pub fn generate_pdf_interne(
        _state: State<DbState>,
        _devis_id: i64,
        _output_path: String,
    ) -> AppResult<()> {
        Err(AppError::Internal("Génération PDF désactivée : recompiler avec --features pdf-generation".to_string()))
    }
}

#[cfg(feature = "pdf-generation")]
pub use pdf_impl::{generate_pdf_client, generate_pdf_interne};

#[cfg(not(feature = "pdf-generation"))]
pub use pdf_fallback::{generate_pdf_client, generate_pdf_interne};
