//! Atualização sem chave de assinatura: baixa o instalador publicado nas releases
//! do GitHub, fecha o programa, instala em modo passivo e abre a nova versão.

const PREFIXO: &str = "https://github.com/FerratoBRq/central-sintramomed/releases/download/";

fn aspas_ps(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

const ULTIMA: &str = "https://github.com/FerratoBRq/central-sintramomed/releases/latest";

/// "https://github.com/.../releases/tag/v0.2.0" -> "0.2.0" (só números: x.y.z).
fn versao_da_url(loc: &str) -> Option<String> {
    let v = loc.rsplit_once("/releases/tag/v")?.1;
    let p: Vec<&str> = v.split('.').collect();
    (p.len() == 3 && p.iter().all(|x| !x.is_empty() && x.len() <= 5 && x.chars().all(|c| c.is_ascii_digit()))).then(|| v.to_string())
}

/// Última versão publicada sem usar a API do GitHub: sem token, a API aceita só 60 consultas por hora por endereço
/// de internet, somando todos os PCs do escritório, e a verificação a cada 5 min estourava ("Sem conexão com o GitHub").
/// Lê o redirecionamento de /releases/latest para /releases/tag/vX.Y.Z e confere que o instalador já foi publicado
/// (o link de download responde 302). Devolve (versão, endereço do instalador) ou None.
#[tauri::command]
pub async fn versao_publicada() -> Result<Option<(String, String)>, String> {
    let client = reqwest::Client::builder()
        .user_agent("CentralSintramomed-atualizador")
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    let r = client.head(ULTIMA).send().await.map_err(|e| format!("Sem conexão com o GitHub: {e}"))?;
    let loc = r.headers().get("location").and_then(|h| h.to_str().ok()).unwrap_or("");
    let Some(v) = versao_da_url(loc) else { return Ok(None) };
    let url = format!("{PREFIXO}v{v}/Central.Sintramomed_{v}_x64-setup.exe");
    let a = client.head(&url).send().await.map_err(|e| format!("Sem conexão com o GitHub: {e}"))?;
    Ok(if a.status().is_redirection() || a.status().is_success() { Some((v, url)) } else { None })
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

    // Espera o instalador terminar (modo silencioso, sem janelas) e abre a nova versão.
    let script = format!(
        "Start-Sleep -Seconds 1; Start-Process -Wait -FilePath {} -ArgumentList '/S'; Start-Process -FilePath {}",
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

#[cfg(test)]
mod testes {
    #[test]
    fn versao_pelo_redirecionamento() {
        assert_eq!(super::versao_da_url("https://github.com/FerratoBRq/central-sintramomed/releases/tag/v0.2.0").as_deref(), Some("0.2.0"));
        assert_eq!(super::versao_da_url("https://github.com/FerratoBRq/central-sintramomed/releases"), None);
        assert_eq!(super::versao_da_url("https://github.com/x/y/releases/tag/v0.2.0/../../z"), None);
        assert_eq!(super::versao_da_url(""), None);
    }
}
