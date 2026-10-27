# Changelog

## 0.1.23 — 27/10/2026
- Afastados e desligados vêm do Sistema eSocial: data de término (tela Término, S-2399) ou de início do afastamento e motivo (S-2230), e a data em que foram enviados. O relatório mensal coloca cada um no mês do envio (o término costuma ser retroativo) e já traz o motivo.
- Lidos na sincronização (uma vez por trabalhador) e no botão Buscar no Sistema eSocial do relatório, para quem saiu nos últimos meses.
- Corrige: a versão 0.1.21 lia a data de cadastro como se fosse a de desligamento.

## 0.1.22 — 27/10/2026
- Relatório mensal: campo para incluir à mão em Admissão (data do cadastro), Periódico (data do periódico) e Afastados/desligados (data e motivo), escolhendo o trabalhador pela busca ou digitando o nome de quem não está no Central.
- Data do ASO da admissão editável; botão Tirar em qualquer linha (as automáticas deixam de aparecer naquele mês).
- Afastados/desligados mostram a data.

## 0.1.21 — 27/10/2026
- Nova aba Aprovação: lançar ASO/NR e editar endereço não gravam mais direto no Sistema eSocial; viram um pedido que mostra cada alteração (aba do sistema, operação, campo, como está hoje e como vai ficar, e quais registros ativos serão marcados INATIVO). Aprovar e enviar (um ou todos) grava e confere relendo o sistema; também dá para recusar. Histórico com quem pediu e quem enviou. Os pedidos são compartilhados entre os computadores.
- ASOs prontos em lote vão para a Aprovação.
- Desligamento e afastamento: a data que aparece no status do Sistema eSocial é lida na sincronização, preenche a data de término da ficha e coloca o trabalhador no relatório mensal do mês certo.

## 0.1.20 — 27/10/2026
- O Central cria a própria pasta (Documentos\Central Sintramomed, ou a escolhida/compartilhada) com "ASOs para lançar", "ASOs lançados" e "Documentos", e mostra onde salvar os ASOs baixados do WhatsApp (botão Abrir a pasta).
- Botão Anexar ASOs no Painel e na tela ASOs recebidos: os arquivos vão para a pasta e são lidos na hora (texto ou OCR), com o trabalhador achado pelo CPF.
- Lançar em lote no Sistema eSocial os ASOs prontos (trabalhador, clínica, data e validade lidos, sem INAPTO), com uma confirmação; os demais, Conferir e lançar.
- Depois de lançado, o ASO é anexado à ficha, o arquivo vai para "ASOs lançados" e uma cópia fica em Documentos\NOME - CPF.
- Retroalimentação: todo anexo da ficha também fica na pasta do trabalhador, e um arquivo colocado à mão nessa pasta entra como anexo (tipo pelo nome: ASO, RG, CPF, CNH, NR-35...). Anexo removido no programa vai para Documentos\_removidos.
- Deixa de vigiar a pasta Downloads (substituído pela pasta do Central).

## 0.1.19 — 27/10/2026
- Nova tela ASOs recebidos: o Central olha a cada minuto a pasta onde o WhatsApp para computador salva os arquivos (normalmente Downloads), lê os PDFs dos últimos 30 dias, reconhece os ASOs (texto ou OCR quando o nome tem "ASO"), acha o trabalhador pelo CPF e deixa na fila com exame, data, resultado e liberações para altura e espaço confinado.
- Conferir e lançar abre a tela de lançamento já com o arquivo; ao confirmar, lança no Sistema eSocial (exame, clínica, data, validade, registro anterior inativo), guarda o PDF na ficha e atualiza as liberações.
- Também dá para arrastar vários PDFs do WhatsApp para a tela. Nada é apagado da pasta.

## 0.1.18 — 27/10/2026
- Listas e inativos: a lista mensal de novos cadastrados virou o relatório mensal no modelo do RH (admissão, periódico e afastados/desligados), com botão Gerar Excel (A4 paisagem, salvo em Downloads).
- Buscar no Sistema eSocial (só leitura): ASO periódicos e admissionais do mês (relatório Atestados/Exames) e a empresa de cada trabalhador pela produção (Folha Produção por trabalhador, tomadora com maior valor).
- Classificação do ASO, envio à DOM Assessoria, antecedentes, empresa e motivo podem ser marcados na tela e ficam guardados (e compartilhados pela pasta).
- Desligados e afastados entram no mês em que o Central percebe a mudança de situação ou em que são marcados para desligar; outros podem ser incluídos à mão.

## 0.1.17 — 27/10/2026
- Pendências: os itens da cadeia (Início, ASO, NR-35, NR-33, Ficha EPI, Produção, Término) viram filtros; clicando, a lista mostra só os trabalhadores com aquela pendência. Todas, Críticas e Atenção mostram a quantidade.

## 0.1.16 — 27/10/2026
- Pendências não mostram mais o aviso DOC "Faltam anexos".

## 0.1.15 — 27/10/2026
- Atalho no painel para o Forms de consulta e inclusão de colaboradores da Coamo.

## 0.1.14 — 27/10/2026
- Nova aba Estoque: camisa manga longa e manga curta (P a XG), calça (36 a 50) e botina (37 a 45), com saldo por tamanho.
- Entradas (compra, devolução) e saídas ligadas ao colaborador: Vale (valor a descontar, com autorização do art. 462 da CLT no termo) e Uniforme da safra (no máximo 1 calça e 1 camisa por colaborador na safra; botina só por vale). Baixa para perda ou avaria.
- Movimentos nunca são apagados: erro se desfaz com Estornar. Vales podem ser marcados como descontados. Termo de entrega A4 para assinar.
- Ficha do trabalhador mostra os uniformes e vales recebidos.
- Engrenagem › Compartilhar entre computadores: com uma pasta do OneDrive, Google Drive ou rede, fichas, anexos, listas, entrevistas, modelos, feedback e estoque ficam iguais em todos os computadores com o Central. Cada computador grava só o próprio arquivo e lê os dos outros a cada minuto.

## 0.1.13 — 26/10/2026
- S-2240 (condições ambientais) não é mais cobrado nas pendências, na cadeia do trabalhador nem na ficha, porque não se aplica à categoria (sugestão #1).

## 0.1.12 — 23/10/2026
- "Boa tarde, …" usa o primeiro nome do login do Sistema eSocial salvo no computador, para cada pessoa que usa o programa ver o próprio nome.

## 0.1.11 — 23/10/2026
- Modelos para impressão: cartão + para adicionar um PDF próprio (nome e descrição) e botão Remover em cada modelo; modelos padrão removidos podem ser restaurados.

## 0.1.10 — 23/10/2026
- Corrige as janelas centrais (Importar, Lançar ASO ou NR, ficha, modelos, anexos) que ficavam atrás do fundo desfocado e não podiam ser usadas.

## 0.1.9 — 23/10/2026
- Visual novo, no estilo Apple: barra lateral translúcida, cartões brancos, tipografia do sistema e azul como cor de destaque, com tema escuro.
- Tela de Feedback: registre sugestões e problemas, envie ao GitHub com um clique e acompanhe o que já foi implementado; notas de cada versão publicada.
- Modelos para impressão abrem dentro do programa, com botão Imprimir (antes não abriam no programa instalado).
- Atualização automática instala em modo silencioso e reabre o programa sozinha.

## 0.1.8 — 23/10/2026
- Lançar ASO ou NR: anexe o documento (PDF com texto ou digitalizado; no programa instalado o texto da imagem é lido por OCR), o Central encontra o trabalhador pelo CPF, lê tipo de exame, data, clínica, resultado e liberações, mostra para conferir e lança no Sistema eSocial (aba Atestados/Exames ou Cursos), marcando o registro anterior como INATIVO. Depois relê o sistema para confirmar e atualiza a ficha.
- "Só registrar no Central" quando não for lançar no sistema.
- Liberações do ASO para altura e espaço confinado na ficha; sem liberação, NR-35/NR-33 deixam de ser cobradas nas pendências.
- Editar o endereço direto no Sistema eSocial (aba Endereço).
- Emitir ficha de cadastro do trabalhador (A4).
- Rodapé com data e hora.

## 0.1.7 — 23/10/2026
- Atualização sozinha pelo GitHub, sem chave de assinatura (como no Livro Caixa): a cada 5 minutos o programa consulta a última versão publicada, baixa o instalador, instala em modo passivo e reabre. Opção "Atualizar sozinho" na engrenagem.
- Cada envio para a main com versão nova gera o instalador automaticamente.

## 0.1.6 — 23/10/2026
- Primeira importação grava em lotes de 25: os trabalhadores aparecem durante a sincronização e o progresso não se perde se o programa fechar.
- Corrige acentos quebrados no programa instalado (página declarada em UTF-8).
- Só uma janela do programa por vez: abrir de novo traz a janela existente para frente.
- Verificação de atualização não mostra mais "sem conexão" quando ainda não há versão nova publicada.

## 0.1.5 — 23/10/2026
- Importação automática no programa instalado: login sozinho no Sistema eSocial (senha guardada no Gerenciador de Credenciais do Windows) e sincronização a cada 5 minutos, só leitura.
- Primeira sincronização importa todos os trabalhadores; depois atualiza produção a cada ciclo e os detalhes (ASO, NR, EPI, documentos) em lotes de 50, priorizando novos e quem mudou de situação.
- Engrenagem com seção Conexões; barra lateral mostra a última sincronização.
- Ícone do programa e workflows de conferência e de release.

## 0.1.4 — 23/10/2026
- Importação do Sistema eSocial (arquivo trabalhadores-esocial.json): cruza por CPF, atualiza status, última produção, ASO, NR-33/35, EPI, documentos e endereço.
- Novos status: afastado e inativo no sistema.
- Cofre de acessos só com Sistema eSocial e Escavador.

## 0.1.3 — 23/10/2026
- Nova interface: tema claro e escuro, barra lateral com ícones, saudação, indicadores e atalhos redesenhados.
- Verificação automática de atualizações a cada 5 minutos (navegador: version.json; .exe: atualizador do GitHub).

## 0.1.2 — 23/10/2026
- Engrenagem de configurações: cofre de logins e senhas criptografado (AES-GCM, PIN de 6+ dígitos), botões Copiar usuário/senha nos atalhos, atalho do Escavador, aviso de novidades da versão.

## 0.1.1 — 23/10/2026
- Verificação de antecedentes na ficha: autorização LGPD, atalhos para Escavador (por nome) e certidão da PF, registro só do resultado.

## 0.1.0 — 23/10/2026
- Etapa 1: painel e atalhos, ficha do trabalhador com anexos, fatiador de PDF, pendências da cadeia S-2300, inativos (+90 dias sem produção), lista mensal de novos cadastrados, entrevista online (protótipo), modelos para impressão A4.
- Estrutura Tauri 2 com atualizador via GitHub Releases e teste no navegador via GitHub Pages.
