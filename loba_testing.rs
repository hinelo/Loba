fn main() {
use std::io::{stdin,stdout,Write};
use std::env;
use std::fs;


let _use_std: Vec<String>;
let use_functions = true;
let use_main = false;
let use_debug = true;
let mut debug = String::new();



let args: Vec<String> = env::args().collect();
let data =  fs::read_to_string(args[1].clone()).expect("Arquivo não encontrado");
println!("{}", data);

let mut criar = ||{
	let mut output = String::new();
	debug += "criar\n"
	if use_functions {
		debug += "use_functions: pergunte\n";
		output +="use std::io::{stdin,stdout,Write};\n\n"
		output += "fn pergunte(conteudo: &str) -> String{"
		output += "let mut variavel =String::new(); "
		output += "print!(\"{}\", conteudo);"
		debug += output;
	} 
	if use_main {
		debug += "use_main = true\n"
	} else {
		output += "\n\n"
		output += &("fn main() {\n" .to_owned() + &data +"\n}" )

	}
	if use_debug {
		println!("{}", debug)
	}

	fs::write(args[1].clone() + ".rs", output).expect("Falha ao gravar o arquivo");
};	

criar()
}