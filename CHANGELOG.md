# Changelog

## 0.1.69 — 29/10/2026
- Programação de pagamento: a opção de valor agora puxa o **valor líquido** de cada trabalhador (o que cai na conta), lido dos holerites das folhas já processadas da quinzena no Sistema eSocial (`rel_folhaPagamento.php` + `holerite2020.php`, só leitura). Empresa sem folha processada fica com o valor em branco e com aviso. Quem trabalhou em mais de uma empresa continua na de mais diárias, com a soma dos líquidos e a rubrica.
- Termo de EPI (Guia): kit padrão conferido com a página real do Guia.

## 0.1.68 — 29/10/2026
- Emitir ficha › **Ordem de Serviço**: gera em Word a O.S. por atividade (modelo Coamo do sindicato) com o nome do trabalhador e as datas de elaboração e de assinatura do dia; a data da última revisão (17/03/2026) e o responsável (Gabriel F. Cavalcante) ficam como no modelo.
- Emitir ficha › **Termo de EPI (Guia)**: sai com o kit padrão (2 calças, 2 camisetas e 1 par de botina) na data do dia, sem as entregas anteriores.

## 0.1.67 — 29/10/2026
- Estoque, Saída ou baixa: ao trocar o motivo, os itens são refeitos. Primeiro uniforme: camiseta manga longa, calça e botina. Uniforme da safra: 1 calça e 1 camiseta manga longa. Demais motivos: 1 camiseta manga longa (a botina do primeiro uniforme não fica mais nas outras opções).

## 0.1.66 — 29/10/2026
- Estoque: novo motivo de saída **Primeiro uniforme (sem custo)**, já com 1 camiseta manga longa, 1 calça e 1 par de botina; só escolher o colaborador e os tamanhos. Uma vez por trabalhador (avisa se já recebeu). Imprime o Termo de entrega do primeiro uniforme e pode lançar na aba EPI's do Sistema eSocial pela Aprovação.

## 0.1.65 — 29/10/2026
- Vencimento das NRs (antes Previsão de vencimentos): só NR-35 e NR-33; o ASO saiu dessa tela (continua em Pendências).
- Calendário com cada mês num cartão com fundo e cabeçalho colorido, dias maiores e o dia de hoje destacado.
- Clique num dia livre marca a turma; num dia marcado abre a turma: quem está nela, mover para outra data, vincular qualquer trabalhador ativo (mesmo quem ainda não está vencendo) e desmarcar a data. Vínculos compartilhados entre os computadores.

## 0.1.64 — 29/10/2026
- **Manual do usuário** na engrenagem (Ajuda), com busca; também pela busca rápida (Ctrl+K).
- Estoque: **Limpar histórico** (aba Configurações) zera saldos e movimentos até agora em todos os computadores, sem apagar os arquivos (corte compartilhado na configuração do estoque).
- Clique duplo não lança, estorna ou marca desconto duas vezes; botões travam enquanto gravam.
- Maiúsculas: mês da Programação de pagamento ("Outubro de 2026"), dias da semana e rótulos como Exemplo, Cadastrado, Sem registro, Estornado, Altura, Confinado, Formação, Reciclagem, Lido em, Até.

## 0.1.63 — 29/10/2026
- Estoque: botão **Contagem** para lançar o inventário. Digita o que há em cada tamanho e o Central lança só a diferença como "Ajuste de inventário".
- Menu: grupo recolhido mostra a soma dos avisos de dentro (ex.: ASOs e NRs esperando em Documentos e eSocial).
- Estoque: botina aparece como "EPI, sem custo" no saldo, sem preço.

## 0.1.62 — 29/10/2026
- Fichas do Guia: o e-mail aparecia como "[email protected]" (proteção do Cloudflare no site do Guia, que depende de um script). Agora o Central decodifica e imprime o e-mail de verdade, do sindicato e do trabalhador.

## 0.1.61 — 29/10/2026
- Emitir ficha: escolha entre a **Ficha do Trabalhador** e o **Termo de EPI** do próprio Sistema eSocial (Guia Campo Grande), mostrados como o Guia emite e impressos pelo Central, ou a ficha do Central.
- Estoque: ao entregar camiseta, calça ou botina (EPI, safra ou desconto autorizado), opção "Lançar também na aba EPI's do Sistema eSocial" (Data, Quantidade e Equipamento, os mesmos campos do Guia). Vai para a Aprovação e, depois de aprovado, é gravado e conferido relendo a aba. Pedido do Gabriel em 29/10/2026.

## 0.1.60 — 29/10/2026
- Novidades resumidas: a engrenagem e o aviso de atualização mostram só uma linha da versão atual; Feedback mostra uma linha por versão. O detalhe fica no histórico completo.

## 0.1.59 — 29/10/2026
- Listas longas em páginas, com 15, 30, 45 ou 60 itens por tela (escolha lembrada) e botões 1, 2, 3…: Trabalhadores, Pendências, Previsão de vencimentos, ASOs e NRs recebidos, Aprovação, Movimentos do estoque, NR-01 e Sem produção.
- Telas longas separadas em abas: Estoque (Saldo, Movimentos, Descontos, Configurações), ASOs/NRs recebidos (Para conferir, Já tratados, Pasta do Central), Aprovação (Esperando aprovação, Histórico) e Listas e inativos (Relatório mensal, Sem produção há +3 meses).
- Estoque, Aprovação e ASOs deixam de cortar a lista (antes mostravam só os 80, 30 ou 20 mais recentes).

## 0.1.58 — 29/10/2026
- NR-01 · Portal DOM em abas (Situação, Importar, Faltam dados), com páginas de 15 ou 30 trabalhadores e botões 1, 2, 3…
- Login do Portal DOM saiu da tela da NR-01 e foi para a engrenagem. Na NR-01 ficam o aviso de sincronia ativa, a última leitura e o botão Sincronizar agora.
- Sincronia com o Portal DOM a cada 10 minutos (antes, 1 hora).
- Botão Importar: salva a planilha (só quem falta no portal) e já abre a tela de importação do portal, para arrastar o arquivo e confirmar.

## 0.1.57 — 29/10/2026
- NR-01 · Portal DOM: acesso ao portal dentro do Central (e-mail e senha guardados no Gerenciador de Credenciais do Windows; só consulta). Sincroniza a lista de Trabalhadores do portal (na hora e a cada hora) e mostra, para cada ativo: fora do portal, convidado, em treinamento ou concluído, situação do convite e itens feitos. Botão para gerar a planilha só dos que faltam no portal.
- Programação de pagamento e NR-01: textos mais curtos (detalhes ao passar o mouse).

## 0.1.56 — 29/10/2026
- Programação de pagamento: opção "Preencher o VALOR com a produção da quinzena". O valor é o Total do extrato de produção (bruto, sem descontos). Quem produziu em mais de uma empresa fica só na empresa onde teve mais diárias (empate: maior valor), com a soma de todas e, logo abaixo, a linha "* Rubrica: soma da produção" com a parte e as diárias de cada empresa. A folha calcula subtotal e total. Precisa clicar em Buscar de novo para trazer os valores.

## 0.1.55 — 29/10/2026
- Nova tela **NR-01 · Portal DOM** (menu Gestão): gera `dom-nr01-ativos.csv` em Downloads no modelo de importação do Portal Psicossocial da DOM Assessoria (nome, CPF, nascimento, cargo Movimentador de Mercadorias, data de início, e-mail e WhatsApp). Lista à parte quem não pode entrar por falta de CPF válido, WhatsApp (celular com DDD) ou data de início. A importação é feita no portal em Trabalhadores › Importar planilha.

## 0.1.54 — 28/10/2026
- Modo noturno: botão com lua (ir para o escuro) ou sol (voltar ao claro) no fim do menu lateral. A escolha fica guardada neste computador; sem escolha, segue o Windows.
- Programação de pagamento: opção "Sem conta corrente ou poupança: puxar a chave PIX" (ligada por padrão). Usa a chave cadastrada nos dados bancários do Sistema eSocial (inclusive quando o tipo de conta é Chave Pix e a chave está no campo da conta); se não houver, o celular ou o e-mail do cadastro, marcado "(conferir)" na folha.

## 0.1.53 — 28/10/2026
- Programação de pagamento: opção de impressão "Uma folha por empresa", "Empresas marcadas na mesma folha" (o nome de cada empresa vem antes dos seus trabalhadores e a linha EMPRESA lista todas) ou "Folha em branco" (só o modelo, com a data para preencher).

## 0.1.52 — 28/10/2026
- Documentos pessoais (CNH, RG, CPF) não geram mais aviso de vencimento em Pendências nem no Painel. Os vencimentos avisados são só ASO, NR-33 e NR-35.

## 0.1.51 — 28/10/2026
- Nova ferramenta **Programação de pagamento**: mês e quinzena (1 a 15 ou 16 ao fim), busca no Sistema eSocial (só leitura) das empresas que tiveram produção no período e de quem produziu em cada uma, com os dados bancários de cada trabalhador (conta corrente, poupança ou PIX). As empresas com produção já vêm marcadas; dá para desmarcar. Imprime a folha A4 no modelo do sindicato (DOURADOS-MS, data, responsável, empresa e período, valores recebidos, DISCRIMINAÇÃO, C/CORRENTE, C/POUPANÇA, VALOR em branco, TOTAL), uma por empresa: poupança, depois corrente, depois PIX e, no fim, quem não tem conta cadastrada. Mais de 27 nomes continuam numa segunda folha.

## 0.1.50 — 28/10/2026
- Nova tela **NRs recebidas** (menu Documentos e eSocial), igual a ASOs recebidos: pasta "NRs para lançar" lida a cada minuto, botão Anexar NRs e arrastar PDFs, leitura do certificado (NR-33 ou NR-35, formação ou reciclagem, carga horária, realização, validade, empresa), trabalhador pelo CPF, "Conferir e lançar" um a um ou "Enviar prontos para aprovação" em lote (prontos = carga no mínimo da norma), contador no menu. Lançados vão para "NRs lançadas".
- ASO colocado na pasta das NRs, ou certificado de NR na pasta dos ASOs, é movido sozinho para a pasta certa.

## 0.1.49 — 28/10/2026
- Novo visual no estilo Liquid Glass (Apple HIG, Materials): o vidro fica só na camada de navegação (menu lateral flutuante, busca rápida); o conteúdo continua em cartões sólidos para manter o contraste. Respeita "reduzir transparência" e "aumentar contraste" do sistema.
- Menu reorganizado por tarefa, com no máximo 2 níveis: Painel; Pessoas (Trabalhadores, Pendências, Vencimentos); Documentos e eSocial (ASOs recebidos, Aprovação); Gestão (Relatório mensal, Estoque e EPIs); Ferramentas (recolhida por padrão). Os grupos recolhem e o programa lembra a escolha neste computador.
- As ações saíram do meio da navegação: botão **Lançar** (ASO, NR-33/NR-35, entrega do estoque), com teclado.
- **Busca rápida** (botão Buscar ou Ctrl+K): trabalhador por nome ou CPF, telas e ações.
- Login do Sistema eSocial: a cidade vai só como "Dourados"; o acesso já salvo como "Dourados MS" é corrigido sozinho.

## 0.1.48 — 28/10/2026
- Revisão jurídica do desconto de uniforme:
  - "Vale" passa a se chamar **Fornecimento com desconto autorizado**. O papel impresso agora é o **Termo de autorização de desconto · Uniforme**, com CPF, preço de custo, autorização prévia, expressa e sem coação (Súmula 342 do TST), parcelas e datas, limite de 70% do valor a receber no período (OJ 18 da SDC do TST) e a declaração de que EPIs e botina são gratuitos.
  - Reposição por perda ou dano exige descrever o que aconteceu; desgaste natural de uso não pode ser descontado (CLT art. 462, § 1º).
  - O termo de entrega do uniforme da safra traz o compromisso prévio de reposição por culpa, que o art. 462, § 1º, da CLT exige.
  - Histórico na folha: "DESC AUTORIZADO UNIFORME". Preços passam a ser "preço de custo"; botina saiu da tabela de preços.
- Entrevista e antecedentes: avisos da Lei 9.029/1995 (estado civil, filhos e saúde não podem ser critério de seleção), da LGPD (saúde é dado sensível; resposta voluntária) e da tese do TST (Tema 1) sobre quando pedir antecedentes.
- Texto do desligamento corrigido ("há mais de 90 dias sem produção") e cargas mínimas das NRs atualizadas no código.

## 0.1.47 — 28/10/2026
- Vale de uniforme ajustado à lei (trabalhador avulso não portuário, Lei 12.023/2009):
  - Botina de segurança e EPIs não podem entrar em vale (CLT art. 166, NR-6 e Lei 12.023/2009 art. 9º: fornecimento gratuito, dever do tomador). Saem por Entrega de EPI, sem custo.
  - O vale exige o motivo: peça extra pedida pelo trabalhador além do uniforme gratuito (autorização prévia por escrito, Súmula 342 do TST) ou reposição por perda/dano causado por ele (CLT art. 462 § 1º). Para peça extra, o trabalhador precisa já ter recebido a calça e a camisa gratuitas da safra.
  - O recibo traz o texto de autorização conforme o motivo e a declaração de que EPIs e botina são sem custo.

## 0.1.46 — 28/10/2026
- Vale uniforme: as datas do desconto seguem o pagamento quinzenal. Cada parcela cai no 5º dia útil (segunda a sexta) da quinzena seguinte: a 1ª na próxima quinzena depois da retirada e a 2ª na quinzena depois dessa. As datas aparecem ao lançar o vale, no recibo e nos pedidos da Aprovação. Feriados não são considerados.

## 0.1.45 — 28/10/2026
- Vale uniforme na folha: ao lançar a saída por vale, cada parcela vira um pedido na aba Aprovação para Lançamentos › Crédito/Desconto do Sistema eSocial, com tipo ADIANTAMENTO SALARIAL (enquanto não existe a rubrica VALE UNIFORME), data (a 2ª parcela no mês seguinte), valor e histórico "VALE UNIFORME - itens - parcela x/n". Depois de aprovado, o Central grava e confere no relatório de Descontos/Créditos.
- O papel do vale passa a ser o Recibo de vale uniforme, com o texto de autorização do desconto em folha (à vista ou em 2 parcelas).
- O Central agora pode gravar também em Crédito/Desconto (só Inserir, sempre pela Aprovação), autorizado pelo Gabriel.

## 0.1.44 — 28/10/2026
- Estoque: o vale pode ser à vista ou em 2 parcelas (valores exatos em centavos).
- Impressão do vale no modelo do sindicato (VALE / VALOR, NOME, DESCRIÇÃO, EMPRESA, DATA dia DE mês DE ano, ASSINATURA): 1 folha A4 com 2 vias e linha de recorte no meio. Vem preenchido com o nome, a descrição dos uniformes com tamanho/número, a empresa da última produção (editável ou em branco), a data da retirada e o valor (com as parcelas); a assinatura fica em branco. Opção de imprimir ou não o valor no cabeçalho.
- Corrige a folha em branco que saía no fim das impressões.
- O vale abre logo depois de lançar a saída por vale e pelo botão Vale na lista de movimentos.

## 0.1.43 — 28/10/2026
- Previsão de vencimentos: botões Lista de certificados NR-35 e NR-33, que geram em Word o modelo "Lista para emissão de certificados" do sindicato (cabeçalho, termo e assinaturas iguais), uma lista por NR. Cada folha mantém as 13 linhas do modelo; com mais nomes, a folha é duplicada com os próximos. Classificação: Reciclagem para quem já tem a NR registrada, Formação para quem não tem. Com uma turma escolhida, sai só ela e com a data da turma.

## 0.1.42 — 28/10/2026
- Previsão de vencimentos: o calendário agora é clicável. Clicar num dia marca (ou desmarca) a turma; com um item escolhido (ASO, NR-35 ou NR-33) marca só dele, em Todos marca para os três. As datas marcadas ficam guardadas e compartilhadas.
- Com datas marcadas, o Central distribui os trabalhadores: vencidos e quem vence antes da 1ª data vão para a 1ª; quem vence depois da 2ª data vai para a 2ª, e assim por diante (sempre a última data marcada antes do vencimento). Sem datas marcadas, continua a sugestão automática.
- Abaixo do calendário, um botão por turma com a quantidade de pessoas, para filtrar a lista.

## 0.1.41 — 28/10/2026
- Logo do SINTRAMOMED na barra lateral, no ícone do programa (atalho, barra de tarefas, instalador) e no cabeçalho das folhas impressas (ficha de cadastro, termo de entrega, previsão de vencimentos).

## 0.1.40 — 28/10/2026
- Previsão de vencimentos (cartão no Painel e item no menu): ASO, NR-35 e NR-33 dos trabalhadores ativos que vencem de hoje até o dia 15 do mês depois do próximo (em outubro, até 15/12), mais os já vencidos.
- Calendário dos 3 meses com as turmas sugeridas só em quarta, quinta e sexta: cada um vai para a data mais tarde possível até 3 dias antes de vencer, juntando o máximo de pessoas por turma (vagas por turma ajustáveis; vencidos vão para a primeira data). Clicar no dia filtra a lista.
- Lista para marcar com telefone, data de vencimento, turma sugerida e campo da data marcada (guardado e compartilhado), e Imprimir lista agrupada por turma.

## 0.1.39 — 28/10/2026
- Lançar NR-33 / NR-35: retirado o campo Data da assinatura (em alguns certificados vinha a data de nascimento).

## 0.1.38 — 28/10/2026
- Menu lateral (Rotina) com os botões Lançar ASO e Lançar NR-33 / NR-35, abertos de qualquer tela.

## 0.1.37 — 28/10/2026
- Botão próprio Lançar NR-33 / NR-35 (Trabalhadores e ficha), separado do Lançar ASO. Lê o certificado em PDF ou foto, inclusive digitalizado de lado (gira a imagem para o OCR), acha o trabalhador pelo CPF e traz NR, Formação ou Reciclagem, carga horária, data de realização, data da assinatura e empresa; validade automática (NR-33 1 ano, NR-35 2 anos).
- Aviso quando a carga não bate com o tipo (NR-33 formação pede 16 h; reciclagem 8 h; NR-35 8 h).
- Vai para a aba Aprovação como o ASO; tipo, carga e assinatura ficam guardados na ficha.
- Lançar ASO passa a ser só para ASO (um certificado de NR anexado ali abre a tela de NR).

## 0.1.36 — 27/10/2026
- Estoque de EPIs: luvas e capa de chuva sem tamanho (tirados G (9), G (11), T (10), P e EXGG); o tamanho deixa de aparecer em negrito. A cinta ergonômica mantém G e XG.

## 0.1.35 — 27/10/2026
- Relatório mensal: em Afastados e desligados a coluna passa a ser "Desligado em", com a data em que o desligamento ou afastamento foi lançado no Sistema eSocial (sempre dentro do mês escolhido). A data de término retroativa deixa de aparecer.

## 0.1.34 — 27/10/2026
- Relatório mensal: afastados e desligados entram no mês em que foram lançados no Sistema eSocial (como na 0.1.23). A data do término do desligamento é retroativa (90 dias sem produção), por isso, pela data do término, os desligados caíam em meses anteriores e o mês ficava só com afastados. A tela mostra a data do término e, abaixo, quando foi lançado.

## 0.1.33 — 27/10/2026
- Compartilhar pelo GitHub: o repositório FerratoBRq/central-sintramomed-dados já vem preenchido. O token continua sendo colado uma vez em cada computador (fica no Gerenciador de Credenciais; nunca vai no programa, que é público).

## 0.1.32 — 27/10/2026
- Corrige o relatório mensal: os desligados não apareciam em Afastados e desligados porque a busca só olhava quem teve produção nos últimos meses (e desligado não aparece no relatório de produção). Agora Buscar no Sistema eSocial lê a data do término de todos os desligados e afastados que ainda não a têm (só na primeira vez; depois fica guardada) e mostra o andamento no botão.
- Corrige a compilação da 0.1.30 (troca pelo GitHub).

## 0.1.31 — 27/10/2026
- Estoque com os itens e tamanhos da planilha "Controle de uniforme no estoque" (sem as quantidades, que estavam desatualizadas): camisetas manga longa e curta M, G, GG, EG e E2G; calça 36 a 50; botina 36 a 45; e 20 EPIs com CA (luvas tátil, petroleira, vaqueta, solda, nitrílica e látex, mangote, toca de soldador, máscara, respirador e filtro RC 203, óculos, protetor auditivo, abafador, protetor facial, capacete, suporte com visor, lanterna, capa de chuva e cinta ergonômica).
- Novo motivo de saída Entrega de EPI (sem desconto), com termo de entrega conforme a NR-6. EPI não entra no uniforme da safra.
- Tela do estoque separada em Uniformes (saldo por tamanho) e EPIs (tabela de saldo).

## 0.1.30 — 27/10/2026
- Compartilhar pelo GitHub (engrenagem): com um repositório PRIVADO só de dados e um token (Contents: Read and write), cada computador envia o que fez (fichas, anexos, aprovações, listas, relatório, entrevistas, modelos, feedback e estoque) e recebe o dos outros a cada minuto. Mão dupla, sem pasta compartilhada.
- O token e o repositório ficam no Gerenciador de Credenciais do Windows; repositório público é recusado (LGPD); o programa só acessa a área de conteúdo desse repositório.

## 0.1.29 — 27/10/2026
- Relatório mensal: afastados e desligados entram só no mês da data do término (S-2399) ou do início do afastamento (S-2230) no Sistema eSocial, como a admissão (data do cadastro) e o periódico (data do exame). Deixa de usar a data de envio e a data em que o Central percebeu a mudança.

## 0.1.28 — 27/10/2026
- Relatório mensal: abre parado em "Mês/Ano", sem escolher um mês sozinho; o relatório aparece depois que o mês é selecionado.

## 0.1.27 — 27/10/2026
- Botão de setas (sincronizar) ao lado da engrenagem, no topo da barra lateral: relê todos os trabalhadores no Sistema eSocial; as setas giram enquanto sincroniza e o andamento aparece embaixo, em porcentagem.
- Ficha de EPI não entregue deixa de ser alerta nas pendências (e sai dos filtros da cadeia).

## 0.1.26 — 27/10/2026
- A cada ciclo de 5 minutos o Central lê os ASOs de todos os trabalhadores no Sistema eSocial (relatório Atestados/Exames, admissional e periódico) e atualiza data e validade. Um ASO lançado por outro computador ou direto no sistema deixa de ficar como vencido no Central (antes podia levar mais de uma hora até a ficha ser relida).

## 0.1.25 — 27/10/2026
- Corrige a sincronização: dados alterados no Sistema eSocial (nome, telefone, e-mail, endereço, matrícula, RG, CTPS, CNH, datas) só entravam no Central quando o campo estava vazio; agora o Sistema eSocial é a fonte e toda mudança é trazida.
- Botão Sincronizar tudo agora (barra lateral e engrenagem): relê todas as fichas de uma vez, com andamento em porcentagem, sem precisar esperar o ciclo automático (que lê 50 fichas a cada 5 minutos).
- Botão Atualizar esta ficha do Sistema eSocial na ficha do trabalhador.

## 0.1.24 — 27/10/2026
- Corrige a tela Lançar ASO ou NR: com ASO escolhido, o campo "Curso" (NR 33/35) aparecia por engano. Campos marcados como ocultos agora somem de verdade em todo o programa.
- Nova Classificação do ASO: Simples ou Completo (altura/espaço confinado), lida do documento. No simples, as liberações ficam desmarcadas e travadas, e NR-35/NR-33 não são cobradas. A classificação vai para a ficha e já preenche o relatório mensal.

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
