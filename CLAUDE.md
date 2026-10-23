# Central Sintramomed — instruções para o Claude

## Regras do projeto
- O Sistema eSocial (sistemaesocial.guiacampogrande.com) é **somente leitura**. Não crie funções que gravem, alterem ou enviem dados lá. O Rust só aceita GET e POST de relatório (`/register/rel_*`).
- Senhas ficam no Gerenciador de Credenciais do Windows (`keyring`). Nunca em arquivo, log ou repositório.
- Textos da interface em português do Brasil, datas dd/mm/aaaa, valores em R$.
- A interface é um HTML único (`web/index.html`) que roda no navegador e dentro do .exe. Use `IS_TAURI` para o que só existe no programa instalado.
- Para lançar versão: subir a versão em `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `APP_VERSION` em `web/index.html` e `web/version.json`; registrar no `CHANGELOG.md`; `git push` na `main`. O GitHub gera o instalador e os PCs se atualizam sozinhos em até 5 min.
- Depois de editar `web/index.html`, conferir a sintaxe do script: `node -e 'const h=require("fs").readFileSync("web/index.html","utf8");new Function(h.split("<script>")[1].split("</script>")[0])'`.

## Código mínimo (adaptado do ponytail, MIT — github.com/DietrichGebert/ponytail)
Escreva só o que a tarefa precisa. Antes de criar código, pare no primeiro degrau que resolve:
1. A funcionalidade é mesmo necessária agora?
2. Já existe no projeto?
3. A biblioteca padrão resolve?
4. Um recurso nativo da plataforma (navegador, Windows, Tauri) resolve?
5. Uma dependência já instalada resolve?
6. Cabe em uma linha?
7. Só então: o mínimo de código que funciona.

- Entenda o problema e corrija a causa, não o sintoma.
- Sem abstrações, camadas ou arquivos "para depois". Prefira apagar a acrescentar.
- Atalho consciente leva comentário `ponytail:` dizendo o limite e quando melhorar.
- **Nunca economize em:** validação de dados de fora, tratamento de erro que evita perda de dados, segurança, acessibilidade básica e o que o usuário pediu explicitamente.
- Lógica não trivial (datas, CPF, parsing do eSocial) ganha uma verificação executável mínima.
