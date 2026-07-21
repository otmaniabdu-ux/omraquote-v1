use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devise {
    pub code: String, // 'SAR', 'USD', 'EUR', 'DZD'
    pub taux_dzd: Decimal,
    pub date_verrouillage: Option<chrono::NaiveDate>,
}