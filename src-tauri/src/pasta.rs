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
    if !ok || !(nome.ends_with(".xlsx") || nome.ends_with(".docx") || nome.ends_with(".csv")) {
        return Err("Nome de arquivo inválido.".into());
    }
    let dir = app.path().download_dir().map_err(|e| format!("Pasta Downloads não encontrada: {e}"))?;
    let destino = dir.join(&nome);
    std::fs::write(&destino, bytes).map_err(|e| format!("Não foi possível salvar em Downloads: {e}"))?;
    Ok(destino.to_string_lossy().to_string())
}

/// Pasta de trabalho do Central: a escolhida (ex.: a pasta compartilhada) ou Documentos\Central Sintramomed.
/// Cria as subpastas "ASOs para lançar", "ASOs lançados", "NRs para lançar", "NRs lançadas" e "Documentos" e devolve o caminho.
#[tauri::command]
pub fn central_pasta(app: tauri::AppHandle, base: String) -> Result<String, String> {
    use tauri::Manager;
    let b = if base.trim().is_empty() {
        app.path().document_dir().map_err(|e| format!("Pasta Documentos não encontrada: {e}"))?.join("Central Sintramomed")
    } else {
        let p = PathBuf::from(base.trim());
        if !p.is_absolute() { return Err("Informe o caminho completo da pasta.".into()); }
        p
    };
    for s in ["ASOs para lançar", "ASOs lançados", "NRs para lançar", "NRs lançadas", "Documentos"] {
        std::fs::create_dir_all(b.join(s)).map_err(|e| format!("Não foi possível criar a pasta {s}: {e}"))?;
    }
    Ok(b.to_string_lossy().to_string())
}

/// Caminho relativo seguro dentro da pasta do Central ("Documentos/NOME - CPF/ASO.pdf").
fn rel_seguro(base: &str, rel: &str) -> Result<PathBuf, String> {
    let b = raiz(base)?;
    let partes: Vec<&str> = rel.split('/').collect();
    let ok = !rel.is_empty() && rel.len() <= 400 && partes.len() <= 4
        && partes.iter().all(|p| !p.is_empty() && *p != "." && *p != ".." && !p.chars().any(|c| "\\:<>\"|?*".contains(c) || c.is_control()));
    if !ok { return Err("Caminho de arquivo inválido.".into()); }
    Ok(partes.iter().fold(b, |acc, p| acc.join(p)))
}

/// Se o arquivo já existir, usa "nome (2).pdf", "nome (3).pdf"...
fn nome_livre(base: &str, rel: &str) -> Result<(PathBuf, String), String> {
    let mut p = rel_seguro(base, rel)?;
    let mut r = rel.to_string();
    let (a, ext) = match rel.rfind('.') { Some(i) if i > rel.rfind('/').unwrap_or(0) => (&rel[..i], &rel[i..]), _ => (rel, "") };
    let mut n = 2;
    while p.exists() { r = format!("{a} ({n}){ext}"); p = rel_seguro(base, &r)?; n += 1; }
    Ok((p, r))
}

/// Arquivos dentro de uma subpasta (até 2 níveis): (caminho relativo com "/", tamanho, modificado em ms).
#[tauri::command]
pub fn arq_listar(base: String, sub: String) -> Result<Vec<(String, u64, u64)>, String> {
    let dir = rel_seguro(&base, &sub)?;
    let mut out = vec![];
    fn andar(d: &std::path::Path, rel: &str, nivel: u8, out: &mut Vec<(String, u64, u64)>) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let nome = e.file_name().to_string_lossy().to_string();
            if nome.starts_with('.') || nome.starts_with('~') || nome.ends_with(".tmp") { continue; }
            let r = format!("{rel}/{nome}");
            let Ok(md) = e.metadata() else { continue };
            if md.is_dir() { if nivel < 2 { andar(&e.path(), &r, nivel + 1, out); } continue; }
            let ms = md.modified().ok().and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64).unwrap_or(0);
            out.push((r, md.len(), ms));
        }
    }
    andar(&dir, &sub, 1, &mut out);
    Ok(out)
}

/// Conteúdo de um arquivo (até 25 MB), entregue como ArrayBuffer.
#[tauri::command]
pub fn arq_ler(base: String, rel: String) -> Result<tauri::ipc::Response, String> {
    let p = rel_seguro(&base, &rel)?;
    let md = std::fs::metadata(&p).map_err(|e| format!("Arquivo não encontrado: {e}"))?;
    if md.len() > 25 * 1024 * 1024 { return Err("Arquivo grande demais (mais de 25 MB).".into()); }
    std::fs::read(&p).map(tauri::ipc::Response::new).map_err(|e| format!("Não foi possível ler o arquivo: {e}"))
}

/// Grava um arquivo (cria as pastas). Não sobrescreve: se o nome existir, devolve o nome livre usado.
#[tauri::command]
pub fn arq_gravar(base: String, rel: String, bytes: Vec<u8>) -> Result<String, String> {
    let (p, r) = nome_livre(&base, &rel)?;
    gravar_seguro(p, &bytes)?;
    Ok(r)
}

/// Move um arquivo dentro da pasta do Central (ex.: de "ASOs para lançar" para "ASOs lançados").
#[tauri::command]
pub fn arq_mover(base: String, de: String, para: String) -> Result<String, String> {
    let o = rel_seguro(&base, &de)?;
    let (d, r) = nome_livre(&base, &para)?;
    if let Some(pai) = d.parent() { std::fs::create_dir_all(pai).map_err(|e| e.to_string())?; }
    std::fs::rename(&o, &d).map_err(|e| format!("Não foi possível mover o arquivo: {e}"))?;
    Ok(r)
}
