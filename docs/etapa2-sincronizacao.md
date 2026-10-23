# Etapa 2 — Sincronização automática com o Sistema eSocial (no .exe)

## O que já foi mapeado (23/10/2026)
Leitura somente, com a sessão logada:
- Lista de todos os trabalhadores: `funcoes/procuraSimples.php?key=%&table=colaborador&field1=cod_colaborador` (722 registros).
- Cadastro: `register/cad_colaborador.php?acao=Procurar&cod_colaborador=ID` (nome, CPF, PIS, nascimento, cadastro, início, status eSocial e status do sistema).
- Abas por trabalhador: `cad_colabEndereco.php`, `colaborador_docs.php` (CTPS, RG, CNH, título), `cad_colabcursos.php` (NR-33, NR-35, NR-11, NR-12, NR-18, NR-31 com validade), `cad_colabexames.php` (ASO admissional/periódico com validade), `cad_coleq.php` (EPIs entregues), `infoEsocial.php` (transmissão do S-2300).
- Última produção: `register/rel_trabInativo.php` (POST `acao=Visualizar&qtdDias=N`) traz o último dia trabalhado com folha de cada ativo.
- Primeira extração: 722 trabalhadores, 182 ativos, 527 desligados, 5 afastados, 8 inativos; 637 com ASO, 525 com NR-35, 180 com produção registrada.

## Como vai funcionar no .exe
1. Primeiro uso: o programa pede login, senha e cidade do Sistema eSocial uma única vez e guarda no Gerenciador de Credenciais do Windows (não fica em arquivo).
2. O próprio programa faz o login e mantém a sessão. Se a sessão cair, entra de novo sozinho.
3. A cada 5 minutos: lista de trabalhadores + relatório de inatividade (poucas requisições). Novos cadastros e mudanças de status entram na hora.
4. Detalhes (ASO, NRs, EPI, documentos): atualizados aos poucos, cerca de 50 trabalhadores por ciclo, e na hora para quem tiver mudança. A volta completa leva cerca de 1 hora, sem sobrecarregar o servidor do sistema.
5. Somente leitura: o programa nunca grava nem envia nada no Sistema eSocial.
6. Painel mostra "Última sincronização às hh:mm" e avisa se o login falhar.

## Implementação
- Rust (Tauri): `reqwest` com cookies, `scraper` para ler o HTML, `keyring` para a senha, banco local SQLite criptografado.
- Comando Tauri `sync_now` + agendador interno de 5 minutos; eventos para a interface atualizar sozinha.
