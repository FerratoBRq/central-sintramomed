// Esconde o console no .exe de produção
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod atualizacao;
mod dom;
mod esocial;
mod github;
mod pasta;

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
        .manage(dom::Dom::new())
        .invoke_handler(tauri::generate_handler![
            esocial::cred_salvar,
            esocial::cred_status,
            esocial::cred_apagar,
            esocial::esocial_testar,
            esocial::esocial_get,
            esocial::esocial_post,
            esocial::esocial_gravar,
            atualizacao::instalar_atualizacao,
            pasta::pasta_ler,
            pasta::pasta_gravar,
            pasta::pasta_arquivo_gravar,
            pasta::pasta_arquivo_ler,
            pasta::salvar_download,
            pasta::central_pasta,
            pasta::arq_listar,
            pasta::arq_ler,
            pasta::arq_gravar,
            pasta::arq_mover,
            github::gh_cred_salvar,
            github::gh_cred_status,
            github::gh_cred_apagar,
            github::gh_conteudo,
            dom::dom_cred_salvar,
            dom::dom_cred_status,
            dom::dom_cred_apagar,
            dom::dom_get
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar a Central Sintramomed");
}
