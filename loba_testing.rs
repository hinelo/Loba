fn main() {
use std::io::{stdin,stdout,Write};
use std::env;
use std::fs;

let mut use_std: Vec<String> = vec![];
let mut use_functions: Vec<String> = vec![];
let use_main = false;
let mut use_debug = true;
let mut debug = String::new();
let args: Vec<String> = env::args().collect();
let mut file_name = String::new();

for n in &args{
	println!("{:?}", args);
	if n.ends_with(".lb") {
		file_name = n.to_string();		
	} else if n.to_string() ==  "-D" ||  n.to_string() == "-d" {
		use_debug = true;
	}
}
if file_name.is_empty() {
	println!("Adicione ao comando, qual arquivo executar");
	return
}

let data =  fs::read_to_string(&file_name).expect("Arquivo não encontrado");

if data.trim().to_lowercase().contains("pergunte") {
	println!("pergunte!");
	use_std.push("std::io::{stdin,stdout,Write};".to_string());
	use_functions.push("pergunte".to_string());
}

if data.trim().to_lowercase().contains("draw") {
	use_functions.push("draw".to_string());
}

if data.trim().to_lowercase().contains("diga!") {
	use_functions.push("diga".to_string());
}

let criar = ||{
	let mut output = String::new();
	debug += "----criando----\n";
	if use_std.is_empty() {
		debug += "use_std = false\n";
	} else {
		debug += &format!("use_std = {:?}\n",use_std).to_string();
		for n in use_std {
			output+= &n;
		}
	}
	if use_functions.is_empty() {
		debug += "use_functions = false\n";
	} else {
		debug += "use_functions = ";
		for n in use_functions {
			if n == "pergunte"{
				let pergunte =  fs::read_to_string("cmd/pergunte").expect("Arquivo não encontrado");
				output += &pergunte;
				debug += " pergunte";
			}
			if n == "diga"{
				output += &fs::read_to_string("cmd/diga").expect("Arquivo não encontrado");
				debug += " diga";
			}
			if n == "draw"{
				output += &fs::read_to_string("cmd/draw").expect("Arquivo não encontrado");
				debug += " draw";
			}
		}
		debug += "\n";
	}

	if use_main {
		debug += "use_main = true\n";
	} else {
		debug += "use_main = false\n";
		output += "\n\n";
		output += &("fn main() {\n" .to_owned() + &data +"\n}" );

	}
	if use_debug {
		debug += "----PRONTO-----";
		println!("{}", debug)
	}

	fs::write(&file_name.replace(".lb", ".rs"), output).expect("Falha ao gravar o arquivo");
};	

criar()





}