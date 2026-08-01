const fs = require('node:fs');
const util = require('util');
const exec = util.promisify(require('child_process').exec);

let source;
let use_debug = true
let std = false

if (process.argv){
    process.argv.forEach(function (val, index, array) {
    if (val.endsWith(".lb")){
      source = val
    }
    if (val == "-nb"){
      use_debug = false
    }
  });
} else {
  console.log("Nenhum arquivo especificado \n\n file.lb : use um arquivo .lb para compilar em loba_espinho\n-nb : use a tag para desativar o debug do loba, erros de compilação ainda serão exibidos.");
} 


let debug = `lendo codigo ${source} \n\n`

let code = fs.readFileSync(source, 'utf8');

debug += code + "\n"

code = code.replace(/diga\((.*?)\)/g, function(match, conteudo){
  std = true
  return 'println!(' + conteudo + ')';
});
code = code.replace(/data(.*?)/g, function(match, conteudo){
  std = true
  return 'struct' + conteudo ;
});
code = code.replace(/função (.*?)\((.*?)\)/g, function(match, nome, conteudo ){
  return 'fn '+ nome +'(' + conteudo + ')';
});

code = code.replace(/pergunte\((.*?)\s*,\s*(.*?)\)/g, function(match, conteudo, variavel ){
  std = true
  return 'let mut '+ variavel +'=String::new(); \n print!('+ conteudo +');\n let _ =stdout().flush(); \n let _ = stdin().read_line(&mut '+ variavel+'); \n'
});

debug += `__________________\nconvertido em:\n\n${code}\n`

let compile = source.replaceAll('lb', "rs");
debug += `__________________\nChecando sependencias:\n\n`

debug += `tem std? = ${std}\n`
code = "use std::io::{stdin,stdout,Write};\n" + code

fs.writeFileSync(compile, "fn main() {\n"+ code +"\n}");


debug += `__________________\n\nrodando:\nrustc' ${compile}\n__________________`

if (use_debug){
  console.log(debug)
}

async function call() {
  const { stdout, stderr } = await exec(`rustc ${compile}`);
  console.log( stdout);
  console.log( stderr);
}
call();
/*
async function run() {
  const { stdout, stderr } = await exec(`./${compile.substring(0, compile.length - 3)}`);
  console.log('stdout:', stdout);
  console.log('stderr:', stderr);
}
run();
*/