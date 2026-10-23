//! Atualização sem chave de assinatura: baixa o instalador publicado nas releases
//! do GitHub, fecha o programa, instala em modo passivo e abre a nova versão.

const PREFIXO: &str = "https://github.com/FerratoBRq/central-sintramomed/releases/download/";

fn aspas_ps(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

#[tauri::command]
pub async fn instalar_atualizacao(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if !url.starts_with(PREFIXO) || !url.ends_with("-setup.exe") || url.contains("..") {
        return Err("Endereço de atualização inválido.".into());
    }
    let client = reqwest::Client::builder()
        .user_agent("CentralSintramomed-atualizador")
        .build()
        .map_err(|e| e.to_string())?;
    let bytes = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Sem conexão para baixar a atualização: {e}"))?
        .error_for_status()
        .map_err(|e| format!("Atualização não encontrada: {e}"))?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let setup = std::env::temp_dir().join("central-sintramomed-setup.exe");
    std::fs::write(&setup, &bytes).map_err(|e| format!("Não foi possível salvar o instalador: {e}"))?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;

    // Espera o instalador terminar (modo passivo, sem perguntas) e abre a nova versão.
    let script = format!(
        "Start-Sleep -Seconds 1; Start-Process -Wait -FilePath {} -ArgumentList '/P'; Start-Process -FilePath {}",
        aspas_ps(&setup.to_string_lossy()),
        aspas_ps(&exe.to_string_lossy())
    );
    let mut cmd = std::process::Command::new("powershell");
    cmd.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-Command", &script]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd.spawn().map_err(|e| format!("Não foi possível iniciar o instalador: {e}"))?;
    app.exit(0);
    Ok(())
}
