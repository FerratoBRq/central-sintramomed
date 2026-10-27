//! Pasta compartilhada (OneDrive, Google Drive para computador ou pasta da rede) para que
//! todos os computadores com o Central vejam as mesmas informações.
//! Cada computador grava só o próprio diário (`<pc>.jsonl`) e lê o de todos, então um
//! computador nunca sobrescreve o que o outro gravou. Anexos vão em `central-dados/arquivos`.

use std::path::PathBuf;

const SUBPASTAS: [&str; 2] = ["central-estoque", "central-dados"];

fn nome_valido(nome: &str, ext: &str) -> bool {
    nome.len() <= 90
        && nome.ends_with(ext)
        && nome.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        && !nome.contains("..")
}

fn raiz(pasta: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(pasta.trim());
    if pasta.trim().is_empty() || !p.is_absolute() {
        return Err("Informe o caminho completo da pasta (ex.: C:\\Users\\voce\\OneDrive\\Central ou \\\\SERVIDOR\\Central).".into());
    }
    if !p.is_dir() {
        return Err("Pasta não encontrada. Confira o caminho e se ela está acessível neste computador.".into());
    }
    Ok(p)
}

fn sub(pasta: &str, subpasta: &str) -> Result<PathBuf, String> {
    if !SUBPASTAS.contains(&subpasta) {
        return Err("Subpasta inválida.".into());
    }
    Ok(raiz(pasta)?.join(subpasta))
}

fn gravar_seguro(destino: PathBuf, conteudo: &[u8]) -> Result<(), String> {
    if let Some(d) = destino.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("Não foi possível criar a pasta: {e}"))?;
    }
    // Grava num temporário e troca, para não deixar arquivo pela metade se faltar energia.
    let tmp = destino.with_extension("tmp");
    std::fs::write(&tmp, conteudo).map_err(|e| format!("Não foi possível gravar na pasta compartilhada: {e}"))?;
    std::fs::rename(&tmp, &destino).map_err(|e| format!("Não foi possível gravar na pasta compartilhada: {e}"))
}

#[tauri::command]
pub fn pasta_ler(pasta: String, subpasta: String) -> Result<Vec<(String, String)>, String> {
    let dir = sub(&pasta, &subpasta)?;
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    for e in std::fs::read_dir(&dir).map_err(|e| format!("Não foi possível ler a pasta: {e}"))? {
        let e = e.map_err(|e| e.to_string())?;
        let nome = e.file_name().to_string_lossy().to_string();
        if nome_valido(&nome, ".jsonl") {
            let txt = std::fs::read_to_string(e.path()).map_err(|e| format!("Não foi possível ler {nome}: {e}"))?;
            out.push((nome, txt));
        }
    }
    Ok(out)
}

#[tauri::command]
pub fn pasta_gravar(pasta: String, subpasta: String, nome: String, conteudo: String) -> Result<(), String> {
    if !nome_valido(&nome, ".jsonl") {
        return Err("Nome de arquivo inválido.".into());
    }
    gravar_seguro(sub(&pasta, &subpasta)?.join(&nome), conteudo.as_bytes())
}

/// Anexo (conteúdo em base64) guardado em central-dados/arquivos/<id>.b64
#[tauri::command]
pub fn pasta_arquivo_gravar(pasta: String, nome: String, base64: String) -> Result<(), String> {
    if !nome_valido(&nome, ".b64") {
        return Err("Nome de arquivo inválido.".into());
    }
    let destino = sub(&pasta, "central-dados")?.join("arquivos").join(&nome);
    if destino.exists() {
        return Ok(()); // anexos não mudam depois de gravados
    }
    gravar_seguro(destino, base64.as_bytes())
}

#[tauri::command]
pub fn pasta_arquivo_ler(pasta: String, nome: String) -> Result<Option<String>, String> {
    if !nome_valido(&nome, ".b64") {
        return Err("Nome de arquivo inválido.".into());
    }
    let p = sub(&pasta, "central-dados")?.join("arquivos").join(&nome);
    if !p.exists() {
        return Ok(None);
    }
    std::fs::read_to_string(p).map(Some).map_err(|e| format!("Não foi possível ler o anexo: {e}"))
}

/// Salva um arquivo gerado pelo programa (ex.: relatório em Excel) na pasta Downloads e devolve o caminho.
#[tauri::command]
pub fn salvar_download(app: tauri::AppHandle, nome: String, bytes: Vec<u8>) -> Result<String, String> {
    use tauri::Manager;
    let ok = nome.len() <= 120
        && !nome.contains("..")
        && nome.chars().all(|c| c.is_alphanumeric() || " -_.()".contains(c));
    if !ok || !nome.ends_with(".xlsx") {
        return Err("Nome de arquivo inválido.".into());
    }
    let dir = app.path().download_dir().map_err(|e| format!("Pasta Downloads não encontrada: {e}"))?;
    let destino = dir.join(&nome);
    std::fs::write(&destino, bytes).map_err(|e| format!("Não foi possível salvar em Downloads: {e}"))?;
    Ok(destino.to_string_lossy().to_string())
}
