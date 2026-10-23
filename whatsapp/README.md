# Atendente do WhatsApp (Kapso + API oficial)

Atendente virtual do WhatsApp do SINTRAMOMED. Responde só com base nos arquivos desta pasta, cita a lei e o artigo, e passa para um atendente humano quando sai do que está escrito.

## Arquivos
- `prompt-operador.md` — regras de conduta do atendente (o que responde, quando passa para humano, o que nunca faz).
- `agente-kapso.json` — configuração do nó de agente no Kapso, já com o roteiro e com este repositório como fonte.
- `base-legal/` — leis vigentes (Lei 12.023/2009, CLT, FGTS, INSS, seguro-desemprego, LGPD, Constituição). Veja `INDICE.md`.
- `base-sindicato/` — informações do sindicato: endereço, documentos, cadastro, regras da ficha de triagem.

Para mudar o que o atendente sabe, edite os arquivos de `base-sindicato/` ou acrescente textos em `base-legal/` e envie para o GitHub. O atendente lê a versão mais nova a cada conversa.

## Custos
- **Kapso, plano Free:** 2.000 mensagens por mês e 1 número conectado (mensagens recebidas e enviadas contam).
- **Meta (WhatsApp):** responder quem escreveu primeiro é gratuito até 1.000 mensagens entregues por número por mês (regra desde 01/10/2026); acima disso, e para mensagens iniciadas pelo sindicato (templates), a Meta cobra por mensagem.

## Configuração (uma vez)
1. Criar conta grátis em kapso.ai e um projeto.
2. Conectar o número do WhatsApp do sindicato pelo link de conexão do Kapso (API oficial da Meta). Para testar antes, use o número sandbox do Kapso.
3. No GitHub, gerar um token somente leitura deste repositório: Settings → Developer settings → Fine-grained tokens → acesso só a `central-sintramomed`, permissão Contents: Read-only.
4. No Kapso, criar um workflow com gatilho "mensagem recebida" e um nó **Agent**:
   - colar o conteúdo de `prompt-operador.md` em System prompt;
   - escolher o modelo;
   - ligar Sandbox e adicionar este repositório como recurso GitHub, com o token do passo 3;
   - ligar as ferramentas complete_task, handoff_to_human, enter_waiting, get_whatsapp_context e get_current_datetime.
   (A mesma configuração está em `agente-kapso.json`.)
5. Testar pelo sandbox com perguntas reais do balcão antes de publicar no número oficial.

## Pelo Claude Code
As skills oficiais do Kapso automatizam os passos 2 a 5:

```
npx -y skills add gokapso/agent-skills --agent claude-code
```

Depois peça ao Claude: "configure o atendente do WhatsApp usando a pasta whatsapp/ deste repositório".

## Falta
- Texto da Convenção Coletiva (CCT) vigente e das NRs (NR-06, NR-07, NR-35, NR-33): adicionar em `base-legal/`.
