//! Consulta ao Portal Psicossocial da DOM Assessoria (portal.domsst.com.br) — NR-01. Só leitura.
//! - E-mail e senha ficam no Gerenciador de Credenciais do Windows (keyring), nunca em arquivo.
//! - O portal é feito em Laravel + Livewire 3: o login é a chamada "login" do componente "auth.login"
//!   (POST JSON em data-update-uri com o snapshot da página e o token data-csrf). Depois disso o
//!   programa só faz GET em páginas da empresa (/empresa/...), com o cookie da sessão em memória.

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

const BASE: &str = "https://portal.domsst.com.br";
const KR_SERVICE: &str = "central-sintramomed";
const KR_USER: &str = "portal-dom";

#[derive(Serialize, Deserialize, Clone)]
struct Cred { email: String, senha: String }

#[derive(Serialize)]
pub struct StatusDom { pub configurado: bool, pub email: String }

pub struct Dom { client: reqwest::Client, logado: Mutex<bool> }

impl Dom {
    pub fn new() -> Self {
        let client = reqwest::Client::builder().cookie_store(true).user_agent("CentralSintramomed/1.0 (leitura)")
            .timeout(std::time::Duration::from_secs(40)).build().expect("cliente HTTP");
        Self { client, logado: Mutex::new(false) }
    }
}

fn entry() -> Result<keyring::Entry, String> { keyring::Entry::new(KR_SERVICE, KR_USER).map_err(|e| e.to_string()) }

fn ler() -> Result<Option<Cred>, String> {
    match entry()?.get_password() {
        Ok(s) => serde_json::from_str(&s).map(Some).map_err(|e| e.to_string()),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Valor de um atributo HTML (primeira ocorrência), já sem as entidades.
fn atributo(html: &str, nome: &str) -> Option<String> {
    let ini = html.find(&format!("{nome}=\""))? + nome.len() + 2;
    let fim = html[ini..].find('"')? + ini;
    Some(html[ini..fim].replace("&quot;", "\"").replace("&#039;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&"))
}

/// Página de login (ou sessão expirada): tem o campo de senha do componente auth.login.
fn e_login(url: &str, html: &str) -> bool { url.ends_with("/login") || html.contains("wire:model=\"password\"") }

async fn fazer_login(st: &Dom, c: &Cred) -> Result<(), String> {
    let html = st.client.get(format!("{BASE}/login")).send().await.map_err(|e| format!("Sem conexão com o Portal DOM: {e}"))?
        .text().await.map_err(|e| e.to_string())?;
    let snapshot = atributo(&html, "wire:snapshot").ok_or("Tela de login do Portal DOM mudou (snapshot não encontrado).")?;
    let csrf = atributo(&html, "data-csrf").ok_or("Tela de login do Portal DOM mudou (token não encontrado).")?;
    let uri = atributo(&html, "data-update-uri").ok_or("Tela de login do Portal DOM mudou (endereço de envio não encontrado).")?;
    if !uri.starts_with(BASE) { return Err("Endereço de login inesperado no Portal DOM.".into()); }
    let corpo = serde_json::json!({"_token": csrf, "components": [{"snapshot": snapshot,
        "updates": {"email": c.email, "password": c.senha, "remember": true},
        "calls": [{"path": "", "method": "login", "params": []}]}]});
    let r = st.client.post(&uri).header("Content-Type", "application/json").header("Accept", "application/json").header("X-Livewire", "")
        .header("Referer", format!("{BASE}/login")).body(corpo.to_string()).send().await.map_err(|e| format!("Sem conexão com o Portal DOM: {e}"))?;
    let status = r.status();
    let txt = r.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() { return Err(format!("Portal DOM recusou o login ({status}).")); }
    let v: serde_json::Value = serde_json::from_str(&txt).map_err(|_| "Resposta inesperada do Portal DOM no login.".to_string())?;
    if v["components"][0]["effects"]["redirect"].is_string() { return Ok(()); }
    Err("E-mail ou senha do Portal DOM não conferem.".into())
}

async fn get_logado(st: &Dom, caminho: &str) -> Result<(String, String), String> {
    let c = ler()?.ok_or("Acesso ao Portal DOM não configurado.")?;
    for tentativa in 0..2 {
        if tentativa == 1 || !*st.logado.lock().await { fazer_login(st, &c).await?; *st.logado.lock().await = true; }
        let r = st.client.get(format!("{BASE}{caminho}")).send().await.map_err(|e| format!("Sem conexão com o Portal DOM: {e}"))?;
        let url = r.url().to_string();
        let html = r.text().await.map_err(|e| e.to_string())?;
        if !e_login(&url, &html) { return Ok((url, html)); }
        *st.logado.lock().await = false;
    }
    Err("Não foi possível entrar no Portal DOM. Confira o e-mail e a senha.".into())
}

/// Confere o login antes de guardar; devolve o caminho do painel da empresa (ex.: /empresa/83).
#[tauri::command]
pub async fn dom_cred_salvar(st: State<'_, Dom>, email: String, senha: String) -> Result<String, String> {
    let c = Cred { email: email.trim().to_string(), senha };
    if !c.email.contains('@') || c.senha.is_empty() { return Err("Informe o e-mail e a senha do Portal DOM.".into()); }
    fazer_login(&st, &c).await?;
    entry()?.set_password(&serde_json::to_string(&c).map_err(|e| e.to_string())?).map_err(|e| format!("Não foi possível guardar no Gerenciador de Credenciais: {e}"))?;
    *st.logado.lock().await = true;
    let (url, _) = get_logado(&st, "/").await?;
    Ok(url.trim_start_matches(BASE).to_string())
}

#[tauri::command]
pub fn dom_cred_status() -> Result<StatusDom, String> {
    Ok(match ler()? { Some(c) => StatusDom { configurado: true, email: c.email }, None => StatusDom { configurado: false, email: String::new() } })
}

#[tauri::command]
pub async fn dom_cred_apagar(st: State<'_, Dom>) -> Result<(), String> {
    *st.logado.lock().await = false;
    match entry()?.delete_credential() { Ok(()) | Err(keyring::Error::NoEntry) => Ok(()), Err(e) => Err(e.to_string()) }
}

/// Leitura de página do portal: "/" (para achar a empresa) ou páginas da empresa (/empresa/...). Devolve (caminho final, HTML).
#[tauri::command]
pub async fn dom_get(st: State<'_, Dom>, caminho: String) -> Result<(String, String), String> {
    let ok = caminho == "/" || (caminho.starts_with("/empresa/") && !caminho.contains("..") && !caminho.contains("://") && caminho.len() <= 300);
    if !ok { return Err("Caminho não permitido no Portal DOM.".into()); }
    let (url, html) = get_logado(&st, &caminho).await?;
    Ok((url.trim_start_matches(BASE).to_string(), html))
}

#[cfg(test)]
mod testes {
    #[test]
    fn atributo_com_entidades() {
        let h = r#"<div wire:snapshot="{&quot;data&quot;:{&quot;email&quot;:&quot;&quot;},&quot;memo&quot;:{&quot;name&quot;:&quot;auth.login&quot;}}" x></div><script data-csrf="abc123" data-update-uri="https://portal.domsst.com.br/livewire-59a3e4f2/update"></script>"#;
        assert_eq!(super::atributo(h, "wire:snapshot").unwrap(), r#"{"data":{"email":""},"memo":{"name":"auth.login"}}"#);
        assert_eq!(super::atributo(h, "data-csrf").unwrap(), "abc123");
        assert!(super::e_login("https://portal.domsst.com.br/login", ""));
    }
}
