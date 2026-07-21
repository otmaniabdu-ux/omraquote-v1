use rust_decimal::Decimal;
use crate::models::devis::Devis;
use crate::models::segment_vol::SegmentVol;
use crate::models::hebergement::Hebergement;
use crate::models::transfert::Transfert;
use crate::models::prestation_vip::PrestationVip;
use crate::services::conversion_devises::convertir_vers_dzd;
use crate::services::calcul_nuitees::calculer_nuitees;
use crate::utils::decimal_helpers::round_to_two_decimals;

/// Calcule le coût net total d'un devis à partir des lignes.
/// Retourne (cout_net_total, montant_marge, prix_vente_total)
pub fn calculer_totaux_devis(
    devis: &Devis,
    segments_vol: &[SegmentVol],
    hebergements: &[Hebergement],
    transferts: &[Transfert],
    prestations: &[PrestationVip],
    // On pourrait ajouter train_haramain plus tard
) -> Result<(Decimal, Decimal, Decimal), String> {
    let mut cout_net_total = Decimal::ZERO;

    // 1. Vols : on prend le prix par passager selon la catégorie.
    // Pour simplifier, on suppose que chaque segment a des prix déjà inclus pour tous les passagers.
    // Mais dans la réalité, il faut multiplier par le nombre de passagers par catégorie.
    // On va additionner tous les prix des segments (pour l'instant on additionne les prix adultes par défaut)
    // TODO: obtenir le nombre de passagers par catégorie depuis le devis.
    for segment in segments_vol {
        let montant_dzd = convertir_vers_dzd(segment.prix_adulte, &segment.devise_prix, devis)?;
        // On multiplie par le nombre de passagers adultes (à récupérer)
        // Pour l'instant, on suppose 1 adulte (mais on doit compter les passagers du devis)
        // En attendant, on ajoute le prix adulte.
        cout_net_total += montant_dzd;
    }

    // 2. Hébergements : on calcule le nombre de nuits et on multiplie par le prix par nuit
    for hebergement in hebergements {
        let nuitees = calculer_nuitees(hebergement.date_checkin, hebergement.date_checkout)?;
        let montant_total_hebergement = hebergement.prix_par_nuit * Decimal::from(nuitees);
        let montant_dzd = convertir_vers_dzd(montant_total_hebergement, &hebergement.devise_prix, devis)?;
        cout_net_total += montant_dzd;
    }

    // 3. Transferts
    for transfert in transferts {
        let montant_total = transfert.prix_unitaire * Decimal::from(transfert.nombre_vehicules);
        let montant_dzd = convertir_vers_dzd(montant_total, &transfert.devise_prix, devis)?;
        cout_net_total += montant_dzd;
    }

    // 4. Prestations VIP
    for prestation in prestations {
        let montant_total = prestation.prix_unitaire * Decimal::from(prestation.quantite);
        let montant_dzd = convertir_vers_dzd(montant_total, &prestation.devise_prix, devis)?;
        cout_net_total += montant_dzd;
    }

    // Appliquer la marge
    let (montant_marge, prix_vente_total) = match devis.marge_type.as_str() {
        "pourcentage" => {
            let marge = cout_net_total * (devis.marge_valeur / Decimal::from(100));
            let prix_vente = cout_net_total + marge;
            (marge, prix_vente)
        }
        "montant_fixe" => {
            let marge = devis.marge_valeur;
            let prix_vente = cout_net_total + marge;
            (marge, prix_vente)
        }
        _ => return Err("Type de marge inconnu".to_string()),
    };

    // Arrondi final
    let cout_net_total = round_to_two_decimals(cout_net_total);
    let montant_marge = round_to_two_decimals(montant_marge);
    let prix_vente_total = round_to_two_decimals(prix_vente_total);

    Ok((cout_net_total, montant_marge, prix_vente_total))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::models::devis::Devis;
    use crate::models::segment_vol::SegmentVol;
    use crate::models::hebergement::Hebergement;
    use crate::models::transfert::Transfert;
    use crate::models::prestation_vip::PrestationVip;
    use chrono::NaiveDate;

    fn create_test_devis() -> Devis {
        Devis {
            id: Some(1),
            numero_devis: "DEVIS-2026-07-001".to_string(),
            client_id: 1,
            date_creation: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            date_depart: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
            date_retour: NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
            type_visa: "omra_standard".to_string(),
            assurance_medicale: true,
            devise_achat: "SAR".to_string(),
            taux_sar_dzd: dec!(30.5),
            taux_usd_dzd: dec!(140.0),
            taux_eur_dzd: dec!(150.0),
            marge_type: "pourcentage".to_string(),
            marge_valeur: dec!(10.0),
            cout_net_total: None,
            montant_marge: None,
            prix_vente_total: None,
            statut: "brouillon".to_string(),
            remise: Some(dec!(0.0)),
            notes_internes: None,
        }
    }

    #[test]
    fn test_calcul_avec_vol_et_hebergement() {
        let devis = create_test_devis();

        let segments_vol = vec![
            SegmentVol {
                id: None,
                devis_id: 1,
                ordre: 1,
                compagnie: "Air Algerie".to_string(),
                numero_vol: Some("AH123".to_string()),
                classe: "economique".to_string(),
                date_vol: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
                aeroport_depart: "ALG".to_string(),
                aeroport_arrivee: "MED".to_string(),
                heure_depart: Some("10:00".to_string()),
                heure_arrivee: Some("14:00".to_string()),
                prix_adulte: dec!(500.0),
                prix_enfant: dec!(300.0),
                prix_bebe: dec!(50.0),
                devise_prix: "SAR".to_string(),
                remarques: None,
            }
        ];

        let hebergements = vec![
            Hebergement {
                id: None,
                devis_id: 1,
                ville: "Makkah".to_string(),
                nom_hotel: "Hilton".to_string(),
                type_chambre: "double".to_string(),
                formule_repas: Some("petit_dejeuner".to_string()),
                vue: Some("Haram".to_string()),
                date_checkin: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
                date_checkout: NaiveDate::from_ymd_opt(2026, 8, 5).unwrap(),
                nb_nuitees: None,
                prix_par_nuit: dec!(200.0),
                devise_prix: "SAR".to_string(),
                taxes_incluses: true,
                remarques: None,
            }
        ];

        let transferts = vec![];
        let prestations = vec![];

        let (cout_net, marge, prix_vente) = calculer_totaux_devis(
            &devis,
            &segments_vol,
            &hebergements,
            &transferts,
            &prestations,
        ).unwrap();

        // Vol: 500 SAR * 30.5 = 15250 DZD
        // Hébergement: 200 SAR * 4 nuits = 800 SAR * 30.5 = 24400 DZD
        // Total net = 39650 DZD
        // Marge 10% = 3965 DZD
        // Prix vente = 43615 DZD
        assert_eq!(cout_net, dec!(39650.0));
        assert_eq!(marge, dec!(3965.0));
        assert_eq!(prix_vente, dec!(43615.0));
    }
}