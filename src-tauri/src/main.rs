// Point d'entrée du binaire Tauri — délègue à lib.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    omravip_quotes::run();
}
