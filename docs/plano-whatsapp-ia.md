# Operador de IA no WhatsApp do SINTRAMOMED — plano (23/10/2026)

## Objetivo
Atender trabalhadores e candidatos no WhatsApp do sindicato com um assistente que só responde com base em fontes legais e documentos do sindicato, cita a fonte, e passa para um atendente humano quando sai do escopo.

## Regras da Meta (o que pode)
- Desde 15/01/2026 a Meta proíbe na API do WhatsApp Business assistentes de IA de uso geral (tipo ChatGPT no WhatsApp). Assistentes de atendimento de uma empresa (dúvidas frequentes, triagem, agendamento, avisos) seguem permitidos.
- Números do Brasil (+55) ficaram fora dessa restrição após inquérito do Cade (aberto em 12/01/2026). Mesmo assim, o projeto deve ficar no formato "atendimento do sindicato" para não depender dessa exceção.

## Caminhos de integração
| Caminho | Situação | Recomendação |
|---|---|---|
| WhatsApp Cloud API oficial (Meta) | Número verificado, templates aprovados, custo por conversa iniciada pela empresa | **Recomendado** |
| Chatwoot (open source) + Cloud API | Caixa de entrada para a equipe, transferência IA → humano, histórico | **Recomendado como painel** |
| n8n (open source) | Orquestra o fluxo: webhook → busca na base legal → IA → resposta | Recomendado |
| Typebot (open source) | Menus e formulários (ex.: entrevista online por WhatsApp) | Opcional |
| APIs não oficiais (Evolution, Baileys, WPPConnect, WAHA) | Contra os termos do WhatsApp, risco de banir o número do sindicato | **Não usar** |

## Base de conhecimento
- `ctresb/leis`: leis federais e estaduais em Markdown com metadados (URN, fonte, situação). Usar só o recorte relevante e filtrar revogadas:
  - Lei 12.023/2009 (avulso não portuário), CLT (trechos aplicáveis), Lei 8.212/91 e 8.213/91 (previdência), Lei 13.709/2018 (LGPD), Lei 7.998/90 (seguro-desemprego), Constituição art. 7º e 8º.
- **Não está no repositório (infralegal pendente):** NR-01, NR-06 (EPI), NR-07 (PCMSO/ASO), NR-11, NR-33, NR-35, manuais do eSocial. Baixar de gov.br e indexar à parte.
- Documentos próprios: CCT vigente, estatuto, tabela de diárias, calendário de pagamento quinzenal, lista de documentos necessários, ficha de triagem.
- O repositório avisa que não substitui o Diário Oficial e tem erros de OCR: toda resposta cita a norma e o artigo, com link da fonte oficial.

## Regras do assistente (guardrails)
1. Responde só com trechos recuperados da base; sem fonte, não responde e encaminha ao atendente.
2. Cita norma, artigo e data em toda resposta jurídica.
3. Não dá parecer individual (ex.: "tenho direito a indenização?"): orienta e agenda com o jurídico do sindicato.
4. Não promete vaga, escala, valor ou prazo que não esteja na CCT ou em documento do sindicato.
5. LGPD: aviso de privacidade na primeira mensagem, não pede documentos sensíveis pelo chat sem consentimento, não informa dados de um trabalhador a outra pessoa.
6. Transferência para humano por palavra-chave ("atendente"), por assunto sensível (acidente, saúde, denúncia) ou após 2 respostas sem fonte.
7. Registro de toda conversa com a fonte usada, para auditoria.

## Arquitetura proposta
WhatsApp Cloud API → Chatwoot (caixa da equipe) → n8n (webhook) → busca na base legal indexada → modelo de IA com as regras acima → resposta com citação → Chatwoot. Integração futura com a Central Sintramomed: consulta de situação do trabalhador (pendências, documentos faltando) mediante confirmação de CPF.

## Etapas
1. Recorte e indexação da base legal + CCT; banco de perguntas reais do balcão.
2. Protótipo de teste (web, sem WhatsApp) para validar respostas e citações.
3. Número na Cloud API + Chatwoot + n8n em servidor (VPS ou nuvem).
4. Piloto com a equipe; depois abertura aos trabalhadores.
