# Central Sintramomed

Programa do setor de Gestão de Pessoas do SINTRAMOMED (Dourados/MS). Complementa o Sistema eSocial: o envio dos eventos continua lá; aqui ficam os trabalhadores sincronizados automaticamente, fichas com anexos, pendências da cadeia S-2300, vencimentos, listas e atalhos.

## Como funciona a importação automática
1. Na primeira abertura do programa, clique em **Conectar agora** e informe login, senha e cidade do Sistema eSocial (uma única vez).
2. A senha fica no Gerenciador de Credenciais do Windows, nunca em arquivo nem no repositório.
3. O programa entra sozinho e, a cada 5 minutos:
   - lê a lista de trabalhadores e o relatório de produção;
   - busca os detalhes (ASO, NR-33/35, EPI, documentos, endereço, S-2300) de quem é novo ou mudou de situação, e atualiza os demais em lotes de 50.
4. Somente leitura: nada é gravado ou enviado no Sistema eSocial.

## Estrutura
```
web/                 Interface (HTML único) — roda no navegador e dentro do .exe
src-tauri/           Programa Windows (Tauri 2)
  src/esocial.rs     Login, sessão e leitura do Sistema eSocial
.github/workflows/
  ci.yml             Confere a compilação a cada envio
  release.yml        Gera o instalador e publica a release a cada versão nova na main
docs/                Mapeamento do sistema e planos
```

## Publicar uma nova versão
1. Atualizar a versão em `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `APP_VERSION` em `web/index.html` e `web/version.json`
2. Registrar no `CHANGELOG.md`
3. Enviar para a main (`git push`). O GitHub gera o instalador e os programas instalados se atualizam sozinhos em até 5 minutos.

## Atualização automática dos programas instalados
Como no Livro Caixa, sem chave: a cada 5 minutos o programa consulta a última release no GitHub; se houver versão nova, baixa o instalador, instala em modo passivo e reabre. Pode ser desligado na engrenagem ("Atualizar sozinho").
