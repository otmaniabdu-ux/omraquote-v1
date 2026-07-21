use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

/// Arrondit un Decimal à 2 décimales avec la stratégie MidpointAwayFromZero (arrondi bancaire)
pub fn round_to_two_decimals(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
}

/// Convertit un f64 en Decimal (uniquement pour des valeurs sûres comme des taux)
pub fn decimal_from_f64(value: f64) -> Decimal {
    Decimal::from_f64(value).expect("Impossible de convertir f64 en Decimal")
}

/// Parse une chaîne en Decimal (pour les entrées utilisateur)
pub fn parse_decimal(s: &str) -> Result<Decimal, String> {
    Decimal::from_str(s).map_err(|e| format!("Erreur de parsing décimal: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_round_to_two_decimals() {
        assert_eq!(round_to_two_decimals(dec!(10.555)), dec!(10.56));
        assert_eq!(round_to_two_decimals(dec!(10.554)), dec!(10.55));
        assert_eq!(round_to_two_decimals(dec!(10.5555)), dec!(10.56));
        assert_eq!(round_to_two_decimals(dec!(10.556)), dec!(10.56));
        assert_eq!(round_to_two_decimals(dec!(10.545)), dec!(10.55)); // MidpointAwayFromZero
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_decimal("123.45").unwrap(), dec!(123.45));
        assert!(parse_decimal("abc").is_err());
    }
}