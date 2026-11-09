---
name: Central Sintramomed
description: Painel de trabalho do setor de Gestão de Pessoas do SINTRAMOMED (sindicato de avulsos, Dourados/MS)
colors:
  bg: "#F5F5F7"
  surface: "#FFFFFF"
  ink: "#1D1D1F"
  muted: "#5E5E63"
  faint: "#6E6E73"
  line: "#E5E5EA"
  accent: "#0071E3"
  accent-text: "#0062C4"
  accent-soft: "#E8F1FD"
  crit: "#D70015"
  crit-soft: "#FDEDEF"
  warn: "#B25000"
  warn-soft: "#FFF4E5"
  ok: "#1E7A34"
  ok-soft: "#EAF7EE"
  dark-bg: "#000000"
  dark-surface: "#1C1C1E"
  dark-ink: "#F5F5F7"
  dark-muted: "#A1A1A6"
  dark-accent: "#0A84FF"
typography:
  display:
    fontFamily: "-apple-system, SF Pro Display, Inter, Segoe UI Variable Display, Segoe UI, system-ui, sans-serif"
    fontWeight: 700
    letterSpacing: "-0.02em"
  body:
    fontFamily: "-apple-system, SF Pro Text, Inter, Segoe UI Variable Text, Segoe UI, system-ui, sans-serif"
    fontSize: "15px"
    lineHeight: 1.47
  label:
    fontSize: "12px"
    fontWeight: 600
  mono:
    fontFamily: "ui-monospace, SF Mono, Menlo, Consolas, monospace"
rounded:
  sm: "10px"
  md: "16px"
  pill: "999px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "#FFFFFF"
    rounded: "{rounded.pill}"
  pill-ok:
    backgroundColor: "{colors.ok-soft}"
    textColor: "{colors.ok}"
  pill-info:
    backgroundColor: "{colors.accent-soft}"
    textColor: "{colors.accent-text}"
---

# Central Sintramomed · sistema visual

## Overview
Estilo macOS "Liquid Glass": barra lateral translúcida em cartão flutuante, conteúdo em cartões brancos sobre cinza claro, tipografia do sistema e azul como única cor de destaque. Modo claro e noturno (lua/sol) com os mesmos papéis de cor. É uma ferramenta de trabalho diário: densidade média, leitura rápida, nada decorativo.

## Colors
- Texto principal `ink`; secundário `muted`; rótulos e cabeçalhos de tabela `faint` (todos com contraste de 4,5:1 ou mais sobre `surface` e `bg`).
- Destaque `accent` só em botão principal, foco e item ativo. Texto azul sobre fundo azul claro usa `accent-text`.
- Estados: `crit` (vencido, erro), `warn` (vence em breve), `ok` (em dia), sempre com o par `-soft` de fundo. Nunca cor como única informação: o texto diz o estado.
- Sem gradientes roxos/ciano. Selos coloridos (atalhos) são escuros o bastante para texto branco (4,5:1).

## Typography
- Corpo 15 px; nenhum texto de leitura abaixo de 12 px; rótulos curtos e contadores no mínimo 11,5 px (exceto documentos impressos, que seguem o modelo de papel).
- Títulos de tela em `display` 700; números de painel grandes; datas sempre dd/mm/aaaa e valores em R$.

## Layout
- Barra lateral com grupos recolhíveis (Pessoas, Documentos e eSocial, Gestão, Ferramentas); em telas baixas (até 820 px) fica compacta e rola com esmaecido.
- Listas longas paginadas (15/30/45/60) e abas no lugar de telas compridas.

## Components
- Botões: primário azul (pílula), secundário cinza claro; ações perigosas com confirmação em dois cliques.
- Cartões (`panel`) com raio 16 px; calendário de vencimentos com fundo por mês (pedido do Gabriel).
- Fichas do Guia impressas exatamente como no Guia; Termo de EPI ajustado a uma folha A4.

## Do's and Don'ts
- Fazer: textos em português do Brasil, frases curtas, botão que diz a ação ("Lançar desconto no Guia").
- Não fazer: texto cinza claro sobre fundo colorido, letra miúda, cartão dentro de cartão sem motivo, ícone decorativo sobre cada título.
