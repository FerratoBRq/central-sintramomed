# Central Sintramomed — instruções para o Claude

## Regras do projeto
- O Sistema eSocial (sistemaesocial.guiacampogrande.com) é onde a informação fica guardada. O Central lê tudo, mas só grava pelos formulários da lista `GRAVAVEIS` em `src-tauri/src/esocial.rs` (Atestados/Exames, Cursos, Endereço), com ação Inserir ou Alterar — nunca Remover —, sempre após confirmação na tela e conferindo depois (relendo a página). Não grave na tela principal do cadastro (gera eventos S-2300/S-2205) sem pedido explícito do Gabriel.
- Compartilhamento entre computadores: pasta escolhida na engrenagem (`cs_pasta`), comandos de `src-tauri/src/pasta.rs`. Cada PC grava só `central-dados/<pc>.jsonl` (diário: vence a alteração mais nova por coleção+chave; trabalhador pela chave CPF) e `central-estoque/<pc>.jsonl` (movimentos do estoque, nunca apagados). Gravação vinda do eSocial ou de outro PC usa `Store.put(...,true)` para não voltar ao diário.
- Relatório mensal (Listas e inativos): `relDoc`/`relLinhas`/`relBuscar`/`relExcel` em `web/index.html`; lê `rel_colabexames.php` (ASO por tipo) e `rel_folhaproducao.php` (empresa por trabalhador) só por POST de relatório; desligados/afastados por `lerSaida` (GET de `cad_termino.php`/`cad_afasttemporario.php?acao=Procurar`, só leitura; mês pelo lançamento no sistema `dtSaidaEnvio` (o término é retroativo), senão `dtSaida`); Excel gerado sem bibliotecas (`xlsxFolha`) e salvo por `salvar_download` (pasta.rs).
- Pasta do Central (`central_pasta`, padrão Documentos\Central Sintramomed ou `cs_central_base`): "ASOs para lançar" é lida a cada minuto (`asoVerificarPasta`, fila `cs_aso_fila`), lançados vão para "ASOs lançados"; "Documentos/NOME - CPF" espelha os anexos nos dois sentidos (`docsSincronizar`, removidos vão para `_removidos`). Arquivos só por `arq_*` em pasta.rs (caminho relativo validado). Lançar no Sistema eSocial, um a um ou em lote, sempre com confirmação.
- Aprovação: todo envio ao Sistema eSocial vira pedido (`aprovCriar`, coleção lists tipo "aprovacao", com `diff` campo/atual/novo) e só é gravado em `aprovEnviar` depois de aprovado na aba Aprovação.
- Troca pelo GitHub: repositório PRIVADO só de dados (`github.rs`: `gh_cred_*` no keyring, recusa público; `gh_conteudo` só em contents/). Na interface, `compLer`/`compGravar`/`compArqGravar`/`compArqLer` usam o GitHub quando configurado, senão a pasta compartilhada. Nunca dados de trabalhadores no repositório público do programa.
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
