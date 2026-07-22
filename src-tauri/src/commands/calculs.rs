use crate::error::{AppResult, AppError};
use rust_decimal::Decimal;
use std::str::FromStr;

#[tauri::command]
pub fn calculer_conversion_backend(
    montant: String,
    devise_source: String,
    taux_sar_dzd: String,
    taux_usd_dzd: String,
    taux_eur_dzd: String,
) -> AppResult<String> {
    let m = Decimal::from_str(&montant).map_err(|e| AppError::Validation(format!("Montant invalide: {}", e)))?;
    let taux = match devise_source.as_str() {
        "SAR" => Decimal::from_str(&taux_sar_dzd).map_err(|e| AppError::Validation(format!("Taux SAR invalide: {}", e)))?,
        "USD" => Decimal::from_str(&taux_usd_dzd).map_err(|e| AppError::Validation(format!("Taux USD invalide: {}", e)))?,
        "EUR" => Decimal::from_str(&taux_eur_dzd).map_err(|e| AppError::Validation(format!("Taux EUR invalide: {}", e)))?,
        "DZD" => Decimal::ONE,
        _ => Decimal::ONE,
    };
    Ok((m * taux).to_string())
}

#[tauri::command]
pub fn calculer_marge_indicative_backend(
    cout_net: String,
    marge_type: String,
    marge_valeur: String,
) -> AppResult<String> {
    let net = Decimal::from_str(&cout_net).map_err(|e| AppError::Validation(format!("Coût net invalide: {}", e)))?;
    let val = Decimal::from_str(&marge_valeur).map_err(|e| AppError::Validation(format!("Valeur marge invalide: {}", e)))?;
    let res = if marge_type == "pourcentage" {
        net * (val / Decimal::from(100))
    } else {
        val
    };
    Ok(res.to_string())
}
