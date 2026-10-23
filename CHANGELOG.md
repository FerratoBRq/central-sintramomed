# Changelog

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
