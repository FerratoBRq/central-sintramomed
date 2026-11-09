// Verificação de design com o detector do Impeccable (github.com/pbakaus/impeccable, Apache-2.0), sem instalar nada no programa.
// Uso: clone o Impeccable e rode, com a pasta web/ servida em http://localhost:8123:
//   NODE_PATH=$(npm root -g) node ferramentas/design-check.js <pasta-do-impeccable>
// Abre cada tela com os dados de exemplo, roda o detector (crates/live/assets/detect-antipatterns-browser.js) e lista os achados.
const {chromium}=require("playwright");
(async()=>{const imp=process.argv[2];if(!imp){console.error("Informe a pasta do Impeccable");process.exit(1)}
  const b=await chromium.launch(),p=await b.newPage({viewport:{width:1366,height:768}});
  await p.route(/fonts.googleapis|cdnjs/,r=>r.abort());await p.goto("http://localhost:8123/index.html");await p.waitForTimeout(2000);
  await p.addScriptTag({path:imp+"/crates/live/assets/detect-antipatterns-browser.js"});
  let total=0;for(const v of ["painel","trabalhadores","pendencias","previsao","estoque","pagamentos","adiantamentos","fatiador","relatorios","aprovacao"]){
    await p.evaluate(v=>{current=v;render()},v);await p.waitForTimeout(400);
    const r=await p.evaluate(async()=>await window.impeccableDetectAsync());
    for(const el of r)for(const f of el.findings){total++;console.log(`${v}\t${f.severity}\t${f.type}\t${el.selector}\t${f.detail}`)}}
  console.log(total?`${total} achado(s)`:"Nenhum achado");await b.close();process.exit(total?2:0)})();
