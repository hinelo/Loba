use std::io::{stdin,stdout,Write};

fn pergunte(conteudo: &str) -> String{
	let mut variavel =String::new(); 
	print!("{}\n", conteudo);
	let _ =stdout().flush(); 
	let _ = stdin().read_line(&mut variavel);
	return variavel.replace("\n", "")
}

fn main() {
	let resposta = pergunte("texto");
	print!("{:?}", resposta);
}