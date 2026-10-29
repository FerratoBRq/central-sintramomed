//! Conexão somente-leitura com o Sistema eSocial (sistemaesocial.guiacampogrande.com).
//!
//! - As credenciais ficam no cofre do sistema operacional (Gerenciador de Credenciais
//!   do Windows), nunca em arquivo.
//! - O programa faz o login sozinho, guarda o cookie da sessão em memória e entra de
//!   novo quando a sessão expira.
//! - Leituras (GET) e POST de relatório. Gravação só pelos formulários da lista
//!   GRAVAVEIS (exames/ASO, cursos/NR, endereço, desconto, EPI), com ação Inserir ou Alterar — nunca Remover —
//!   e sempre depois de confirmação na tela.

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

const HOST: &str = "sistemaesocial.guiacampogrande.com";
const BASE: &str = "https://sistemaesocial.guiacampogrande.com";
const KR_SERVICE: &str = "central-sintramomed";
const KR_USER: &str = "sistema-esocial";

#[derive(Serialize, Deserialize, Clone)]
pub struct Credenciais {
    pub login: String,
    pub senha: String,
    pub cidade: String,
}

#[derive(Serialize)]
pub struct StatusCred {
    pub configurado: bool,
    pub login: String,
    pub cidade: String,
}

pub struct Esocial {
    client: reqwest::Client,
    logado: Mutex<bool>,
}

impl Esocial {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .user_agent("CentralSintramomed/1.0 (leitura)")
            .timeout(std::time::Duration::from_secs(40))
            .build()
            .expect("cliente HTTP");
        Self { client, logado: Mutex::new(false) }
    }
}

fn entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(KR_SERVICE, KR_USER).map_err(|e| e.to_string())
}

fn ler_cred() -> Result<Option<Credenciais>, String> {
    match entry()?.get_password() {
        Ok(s) => serde_json::from_str(&s).map(Some).map_err(|e| e.to_string()),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Converte a resposta em texto: tenta UTF-8 e cai para Windows-1252 (páginas antigas em Latin-1).
fn decodificar(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => encoding_rs::WINDOWS_1252.decode(bytes).0.into_owned(),
    }
}

/// A página de login tem o campo userSenha; se ele aparece, a sessão não está ativa.
fn e_tela_login(html: &str) -> bool {
    html.contains("name=\"userSenha\"") || html.contains("name='userSenha'")
}

/// O Sistema eSocial espera só o nome da cidade ("Dourados"); tira a UF do fim ("Dourados MS", "Dourados/MS", "Dourados - MS").
fn so_cidade(c: &str) -> String {
    let t = c.trim();
    let sem = t.strip_suffix("MS").or_else(|| t.strip_suffix("ms")).map(|r| r.trim_end_matches(|ch: char| ch == ' ' || ch == '/' || ch == '-')).unwrap_or(t);
    if sem.is_empty() { t.to_string() } else { sem.trim().to_string() }
}

async fn fazer_login(st: &Esocial) -> Result<(), String> {
    let c = ler_cred()?.ok_or("Acesso ao Sistema eSocial não configurado. Abra a engrenagem e salve login, senha e cidade.")?;
    let cidade = so_cidade(&c.cidade);
    let form = [
        ("userLogin", c.login.as_str()),
        ("userSenha", c.senha.as_str()),
        ("userCidade", cidade.as_str()),
        ("btnAcessar", "Acessar"),
    ];
    st.client
        .post(format!("{BASE}/"))
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("Sem conexão com o Sistema eSocial: {e}"))?;
    // Confirma abrindo a tela principal: logado, ela fica no próprio sistema e tem o menu.
    let r = st
        .client
        .get(format!("{BASE}/publico/main.php"))
        .send()
        .await
        .map_err(|e| format!("Sem conexão com o Sistema eSocial: {e}"))?;
    let mesmo_site = r.url().host_str() == Some(HOST);
    let main = decodificar(&r.bytes().await.map_err(|e| e.to_string())?);
    let ok = mesmo_site && !e_tela_login(&main) && main.contains("navegar(");
    *st.logado.lock().await = ok;
    if ok {
        Ok(())
    } else {
        Err("Login recusado pelo Sistema eSocial. Confira login, senha e cidade.".into())
    }
}

/// Sem sessão o sistema redireciona para o portal (outro endereço) ou mostra a tela de login.
fn sessao_caiu(r: &reqwest::Response) -> bool {
    r.url().host_str() != Some(HOST)
}

async fn pedir(st: &Esocial, req: impl Fn() -> reqwest::RequestBuilder) -> Result<String, String> {
    if !*st.logado.lock().await {
        fazer_login(st).await?;
    }
    for tentativa in 0..2 {
        let r = req().send().await.map_err(|e| format!("Falha de rede: {e}"))?;
        let caiu = sessao_caiu(&r);
        let html = decodificar(&r.bytes().await.map_err(|e| e.to_string())?);
        if !caiu && !e_tela_login(&html) {
            return Ok(html);
        }
        if tentativa == 0 {
            fazer_login(st).await?; // sessão expirou: entra de novo e repete
        }
    }
    Err("A sessão do Sistema eSocial não abriu. Tente salvar o acesso de novo.".into())
}

fn url(caminho: &str) -> Result<String, String> {
    // Só caminhos do próprio sistema, nunca outro site.
    if !caminho.starts_with('/') || caminho.contains("..") || caminho.contains("://") {
        return Err("Caminho inválido".into());
    }
    Ok(format!("{BASE}{caminho}"))
}

#[tauri::command]
pub fn cred_salvar(login: String, senha: String, cidade: String) -> Result<(), String> {
    let c = Credenciais { login: login.trim().into(), senha, cidade: so_cidade(&cidade) };
    if c.login.is_empty() || c.senha.is_empty() || c.cidade.is_empty() {
        return Err("Preencha login, senha e cidade.".into());
    }
    entry()?.set_password(&serde_json::to_string(&c).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cred_status() -> Result<StatusCred, String> {
    Ok(match ler_cred()? {
        Some(c) => StatusCred { configurado: true, login: c.login, cidade: so_cidade(&c.cidade) },
        None => StatusCred { configurado: false, login: String::new(), cidade: String::new() },
    })
}

#[tauri::command]
pub async fn cred_apagar(st: State<'_, Esocial>) -> Result<(), String> {
    *st.logado.lock().await = false;
    match entry()?.delete_credential() {
        Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn esocial_testar(st: State<'_, Esocial>) -> Result<(), String> {
    *st.logado.lock().await = false;
    fazer_login(&st).await
}

#[tauri::command]
pub async fn esocial_get(st: State<'_, Esocial>, caminho: String) -> Result<String, String> {
    let u = url(&caminho)?;
    pedir(&st, || st.client.get(&u)).await
}

#[tauri::command]
pub async fn esocial_post(st: State<'_, Esocial>, caminho: String, campos: Vec<(String, String)>) -> Result<String, String> {
    // POST permitido apenas para relatórios (consulta), nunca para cadastros.
    if !caminho.starts_with("/register/rel_") {
        return Err("Somente relatórios podem ser consultados por POST".into());
    }
    let u = url(&caminho)?;
    pedir(&st, || st.client.post(&u).form(&campos)).await
}

/// Formulários em que o Central pode gravar (ASO, cursos de NR, endereço e desconto do vale uniforme).
const GRAVAVEIS: [&str; 5] = [
    "/register/cad_colabexames.php",
    "/register/cad_colabcursos.php",
    "/register/cad_colabEndereco.php",
    // Desconto do vale uniforme na folha (Lançamentos › Crédito/Desconto), autorizado pelo Gabriel em 28/10/2026.
    "/register/cad_desconto.php",
    // Aba EPI's do trabalhador (entrega de camiseta, calça e botina), pedido do Gabriel em 29/10/2026.
    "/register/cad_coleq.php",
];

#[tauri::command]
pub async fn esocial_gravar(st: State<'_, Esocial>, caminho: String, campos: Vec<(String, String)>) -> Result<String, String> {
    let pagina = caminho.split('?').next().unwrap_or("");
    if !GRAVAVEIS.contains(&pagina) {
        return Err("Este formulário não pode ser gravado pelo Central.".into());
    }
    let acao = campos.iter().find(|(k, _)| k == "acao").map(|(_, v)| v.as_str()).unwrap_or("");
    if acao != "Inserir" && acao != "Alterar" {
        return Err("Ação não permitida (só Inserir ou Alterar).".into());
    }
    let u = url(&caminho)?;
    pedir(&st, || st.client.post(&u).form(&campos)).await
}

#[cfg(test)]
mod testes {
    #[test]
    fn cidade_sem_uf() {
        for e in ["Dourados MS", "Dourados/MS", "Dourados - MS", "Dourados", " Dourados ms "] { assert_eq!(super::so_cidade(e), "Dourados"); }
    }
}
