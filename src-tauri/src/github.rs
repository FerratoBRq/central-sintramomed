//! Troca de dados entre os computadores por um repositório PRIVADO do GitHub (só de dados).
//! - Repositório e token ficam no Gerenciador de Credenciais do Windows (keyring), nunca em arquivo.
//! - Só aceita repositório privado (dados de trabalhadores: LGPD).
//! - Só acessa a pasta `contents/` desse repositório; cada computador grava o próprio arquivo.

use serde::{Deserialize, Serialize};

const KR_SERVICE: &str = "central-sintramomed";
const KR_USER: &str = "github-dados";
const API: &str = "https://api.github.com/repos/";

#[derive(Serialize, Deserialize)]
struct Cred { repo: String, token: String }

#[derive(Serialize)]
pub struct StatusGh { pub configurado: bool, pub repo: String }

fn entry() -> Result<keyring::Entry, String> { keyring::Entry::new(KR_SERVICE, KR_USER).map_err(|e| e.to_string()) }

fn ler() -> Result<Option<Cred>, String> {
    match entry()?.get_password() {
        Ok(s) => serde_json::from_str(&s).map(Some).map_err(|e| e.to_string()),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn cliente() -> Result<reqwest::Client, String> {
    reqwest::Client::builder().user_agent("CentralSintramomed/1.0").timeout(std::time::Duration::from_secs(120)).build().map_err(|e| e.to_string())
}

fn repo_valido(r: &str) -> bool {
    let p: Vec<&str> = r.split('/').collect();
    p.len() == 2 && p.iter().all(|s| !s.is_empty() && s.len() <= 100 && s.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c)) && *s != "." && *s != "..")
}

/// Confere o acesso e se o repositório é privado antes de guardar.
#[tauri::command]
pub async fn gh_cred_salvar(repo: String, token: String) -> Result<(), String> {
    let repo = repo.trim().trim_start_matches("https://github.com/").trim_end_matches('/').trim_end_matches(".git").to_string();
    let token = token.trim().to_string();
    if !repo_valido(&repo) { return Err("Informe o repositório como dono/nome (ex.: FerratoBRq/central-sintramomed-dados).".into()); }
    if token.len() < 20 { return Err("Token inválido.".into()); }
    let r = cliente()?.get(format!("{API}{repo}")).bearer_auth(&token).header("Accept", "application/vnd.github+json").send().await.map_err(|e| format!("Sem conexão com o GitHub: {e}"))?;
    if r.status().as_u16() == 404 || r.status().as_u16() == 401 { return Err("Repositório não encontrado ou token sem acesso a ele.".into()); }
    if !r.status().is_success() { return Err(format!("GitHub respondeu {}", r.status())); }
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    if v["private"].as_bool() != Some(true) { return Err("Esse repositório é público. Os dados dos trabalhadores só podem ir para um repositório PRIVADO.".into()); }
    if v["permissions"]["push"].as_bool() == Some(false) { return Err("O token não tem permissão de escrita nesse repositório (Contents: Read and write).".into()); }
    let s = serde_json::to_string(&Cred { repo, token }).map_err(|e| e.to_string())?;
    entry()?.set_password(&s).map_err(|e| format!("Não foi possível guardar no Gerenciador de Credenciais: {e}"))
}

#[tauri::command]
pub fn gh_cred_status() -> Result<StatusGh, String> {
    Ok(match ler()? { Some(c) => StatusGh { configurado: true, repo: c.repo }, None => StatusGh { configurado: false, repo: String::new() } })
}

#[tauri::command]
pub fn gh_cred_apagar() -> Result<(), String> {
    match entry()?.delete_credential() { Ok(()) | Err(keyring::Error::NoEntry) => Ok(()), Err(e) => Err(e.to_string()) }
}

/// Chamada à API de conteúdo do repositório configurado: metodo GET ou PUT, caminho dentro do repositório.
/// raw=true devolve o conteúdo do arquivo como texto. Devolve (status, corpo).
#[tauri::command]
pub async fn gh_conteudo(metodo: String, caminho: String, corpo: Option<String>, raw: bool) -> Result<(u16, String), String> {
    let c = ler()?.ok_or("GitHub não configurado.")?;
    let ok = !caminho.is_empty() && caminho.len() <= 300
        && caminho.split('/').all(|p| !p.is_empty() && p != "." && p != ".." && p.chars().all(|ch| ch.is_ascii_alphanumeric() || "-_.".contains(ch)));
    if !ok { return Err("Caminho inválido.".into()); }
    let url = format!("{API}{}/contents/{caminho}", c.repo);
    let cl = cliente()?;
    let req = match metodo.as_str() {
        "GET" => cl.get(&url),
        "PUT" => cl.put(&url).body(corpo.unwrap_or_default()),
        _ => return Err("Método não permitido.".into()),
    };
    let r = req.bearer_auth(&c.token).header("Accept", if raw { "application/vnd.github.raw" } else { "application/vnd.github+json" })
        .header("X-GitHub-Api-Version", "2022-11-28").send().await.map_err(|e| format!("Sem conexão com o GitHub: {e}"))?;
    let st = r.status().as_u16();
    Ok((st, r.text().await.map_err(|e| e.to_string())?))
}
