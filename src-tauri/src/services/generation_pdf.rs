use std::collections::HashMap;
use std::sync::Arc;
use std::io::Write;

use comemo::Tracked;
use ecow::EcoString;
use serde_json::{json, Value};

use typst::diag::{At, SourceResult, StrResult};
use typst::eval::{Library, Tracer};
use typst::file::FileId;
use typst::foundations::{Bytes, Datetime, Smart};
use typst::layout::Frame;
use typst::model::Document;
use typst::syntax::{FileId, Source, World};
use typst::text::{Font, FontBook};
use typst::WorldExt;
use typst_pdf::{PdfOptions, PdfStandard};

// Une structure pour servir de "monde" Typst avec les polices et les fichiers
struct CustomWorld {
    fonts: Vec<Font>,
    book: FontBook,
    main: Source,
    files: HashMap<FileId, Source>,
}

impl CustomWorld {
    fn new(main_source: &str, fonts_data: Vec<(&'static [u8], &'static str)>) -> Self {
        let mut fonts = Vec::new();
        let mut book = FontBook::default();

        for (data, name) in fonts_data {
            let font = Font::new(Bytes::from_static(data), 0).expect("Impossible de charger la police");
            fonts.push(font.clone());
            book.push(font);
        }

        // Source principale
        let file_id = FileId::new(None, EcoString::from("main.typ"));
        let main = Source::new(file_id, EcoString::from(main_source));

        Self {
            fonts,
            book,
            main,
            files: HashMap::new(),
        }
    }
}

impl World for CustomWorld {
    fn library(&self) -> &Library {
        // Utiliser la bibliothèque standard
        &typst::library::STANDARD_LIBRARY
    }

    fn book(&self) -> &FontBook {
        &self.book
    }

    fn main(&self) -> &Source {
        &self.main
    }

    fn source(&self, id: FileId) -> StrResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else {
            // On pourrait ajouter d'autres fichiers (images, etc.) ici
            Err(anyhow::anyhow!("Fichier non trouvé").into())
        }
    }

    fn file(&self, id: FileId) -> StrResult<Bytes> {
        // Pour les images, on pourrait charger depuis le système, mais on n'en a pas besoin
        Err(anyhow::anyhow!("Fichier non trouvé").into())
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(2026, 7, 16)?)
    }
}

/// Génère un PDF à partir d'un template Typst et de données JSON.
pub fn generer_pdf(
    template_typ: &str,
    donnees_json: Value,
    polices: Vec<(&'static [u8], &'static str)>,
) -> Result<Vec<u8>, String> {
    // Construire le monde avec les polices et le template
    let world = CustomWorld::new(template_typ, polices);

    // Compiler le document
    let result = typst::compile(&world);
    match result {
        Ok(doc) => {
            // Générer le PDF
            let pdf_options = PdfOptions {
                standard: PdfStandard::default(),
                timestamp: Smart::Auto,
            };
            let pdf_bytes = typst_pdf::pdf(&doc, &pdf_options)
                .map_err(|e| format!("Erreur de rendu PDF: {}", e))?;
            Ok(pdf_bytes)
        }
        Err(diagnostics) => {
            // Afficher les erreurs de compilation Typst
            let errors: Vec<String> = diagnostics
                .iter()
                .map(|d| format!("{}", d.message))
                .collect();
            Err(format!("Erreurs Typst: {}", errors.join("\n")))
        }
    }
}