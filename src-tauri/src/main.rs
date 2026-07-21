#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod db;
mod services;
mod utils;

use db::connection::{init_db, run_migrations};
use db::DbState;
use tauri::Manager;

// Importer les commandes
use commands::clients::{
    create_client, get_client_by_id, list_clients, update_client, delete_client,
    generate_client_code,
};
use commands::devis::{
    create_devis, get_devis_by_id, list_devis, update_devis, delete_devis,
    calculate_totals, get_alertes_devis,
};
use commands::passagers::{
    create_passager, get_passager_by_id, list_passagers_by_devis, delete_passager,
};
use commands::validation::{
    valider_dates_devis_command,
    valider_hebergement_command,
    get_passeport_alertes,
    check_passager_passeport,
};
use commands::hotels::{
    create_hotel, get_hotel_by_id, list_hotels, update_hotel, delete_hotel,
};
use commands::compagnies::{
    create_compagnie, get_compagnie_by_id, list_compagnies, update_compagnie, delete_compagnie,
};
#[cfg(feature = "pdf-generation")]
use commands::pdf::{generate_pdf_client, generate_pdf_interne};

// Re-export des commandes PDF (également disponibles via pdf.rs quand feature non activée)
#[cfg(not(feature = "pdf-generation"))]
use commands::pdf::{generate_pdf_client, generate_pdf_interne};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let conn = init_db(&app_handle).expect("Echec de connexion a la base");
            run_migrations(&conn).expect("Echec des migrations");

            app.manage(DbState(std::sync::Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Clients
            create_client,
            get_client_by_id,
            list_clients,
            update_client,
            delete_client,
            generate_client_code,
            // Devis
            create_devis,
            get_devis_by_id,
            list_devis,
            update_devis,
            delete_devis,
            calculate_totals,
            get_alertes_devis,
            // Passagers
            create_passager,
            get_passager_by_id,
            list_passagers_by_devis,
            delete_passager,
            // Validation
            valider_dates_devis_command,
            valider_hebergement_command,
            get_passeport_alertes,
            check_passager_passeport,
            // PDF
            generate_pdf_client,
            generate_pdf_interne,
            // Hotels (catalogue)
            create_hotel,
            get_hotel_by_id,
            list_hotels,
            update_hotel,
            delete_hotel,
            // Compagnies
            create_compagnie,
            get_compagnie_by_id,
            list_compagnies,
            update_compagnie,
            delete_compagnie,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'execution de Tauri");
}
