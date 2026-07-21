// Dans generation_pdf.rs, on peut définir une fonction qui retourne les polices
fn charger_polices() -> Vec<(&'static [u8], &'static str)> {
    vec![
        (include_bytes!("../../assets/fonts/PlayfairDisplay-Regular.ttf"), "Playfair Display"),
        (include_bytes!("../../assets/fonts/PlayfairDisplay-Bold.ttf"), "Playfair Display"),
        (include_bytes!("../../assets/fonts/Lato-Regular.ttf"), "Lato"),
        (include_bytes!("../../assets/fonts/Lato-Bold.ttf"), "Lato"),
        (include_bytes!("../../assets/fonts/Amiri-Regular.ttf"), "Amiri"),
        (include_bytes!("../../assets/fonts/Amiri-Bold.ttf"), "Amiri"),
    ]
}