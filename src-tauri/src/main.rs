// Esconde o console no .exe de produção
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod esocial;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .manage(esocial::Esocial::new())
        .invoke_handler(tauri::generate_handler![
            esocial::cred_salvar,
            esocial::cred_status,
            esocial::cred_apagar,
            esocial::esocial_testar,
            esocial::esocial_get,
            esocial::esocial_post
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar a Central Sintramomed");
}
