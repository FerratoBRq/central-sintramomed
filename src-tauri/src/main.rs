// Esconde o console no .exe de produção
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod atualizacao;
mod esocial;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // Uma janela só: abrir o programa de novo traz a janela existente para frente.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
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
            esocial::esocial_post,
            atualizacao::instalar_atualizacao
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar a Central Sintramomed");
}
