use std::io::{stdin,stdout,Write}; 
fn pergunte(conteudo: &str) -> String{
	let mut variavel =String::new(); 
	print!("{}\n", conteudo);
	let _ =stdout().flush(); 
	let _ = stdin().read_line(&mut variavel);
	return variavel.replace("\n", "")
}
fn draw<const L: usize, const C: usize>(matrix: &[[u8; C]; L]) {
    for double in matrix.chunks(2) {
        if double.len() == 2 {
            for (top, bot) in double[0].iter().zip(double[1].iter()) {
                 print!("\x1b[38;5;{}m\x1b[48;5;{}m▀", top, bot);
            }
            print!("\x1b[0m\n");
        } else {
            for top in double[0].iter() {
                 print!("\x1b[38;5;{}m▀", top);
            }
            print!("\x1b[0m\n");
        }   
    }
}
macro_rules! diga {
    () => {
        println!();
    };

    (? $head:expr, $($tail:tt)*) => {
        print!("{:?} ", $head);    
        diga!($($tail)*);  
    };

    (? $head:expr) => {
        println!("{:?}", $head);
    };

    ($head:expr, $($tail:tt)*) => {
        print!("{} ", $head);      
        diga!($($tail)*);  
    };

    ($head:expr) => {
        println!("{}", $head)
    };
}


fn main() {
use std::env;
use std::fs;
use std::process::Command;

let mut use_std: Vec<String> = vec![];
let mut use_functions: Vec<String> = vec![];
let mut use_main = false;
let mut use_debug = false;
let mut _sskeep_file = true;
let mut debug = String::new();
let args: Vec<String> = env::args().collect();
let mut file_name = String::new();

for n in &args{
	if n.ends_with(".lb") {
		file_name = n.to_string();		
	} else if n.to_string() ==  "-D" ||  n.to_string() == "-d" {
		use_debug = true;
	} else if n.to_string() ==  "-F" ||  n.to_string() == "-f" {
		use_debug = true;
	}
}

if file_name.is_empty() {
	diga!("Adicione ao comando, qual arquivo executar");
	return
}

let cwd = std::env::current_dir().unwrap();
let data = fs::read_to_string(cwd.join(&file_name)).unwrap_or_else(|erro| {
    panic!(
        "Arquivo não encontrado em: {:?}\nMotivo: {}\n", 
        cwd.join(&file_name), erro
    );
})
.trim()
.replace("função", "fn");

if data.trim().contains("main(") {
	use_main = false
}

if data.trim().contains("pergunte(") {
	use_std.push("std::io::{stdin,stdout,Write};".to_string());
	use_functions.push("pergunte".to_string());
}

if data.trim().contains("draw(") {
	use_functions.push("draw".to_string());
}

if data.trim().contains("diga!(") {
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
			 output += &format!("use {} \n", n);
		}
	}

	if use_functions.is_empty() {
		debug += "use_functions = false\n";
	} else {
		debug += "use_functions = ";
		for n in use_functions {
	        if n == "pergunte" {
	            output += include_str!("cmd/pergunte");
	            debug += " pergunte";
	        }
	        if n == "diga" {
	            output += include_str!("cmd/diga");
	            debug += " diga";
	        }
	        if n == "draw" {
	            output += include_str!("cmd/draw");
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
		diga!(debug);
	}

	fs::write(&file_name.replace(".lb", ".rs"), output).expect("Falha ao gravar o arquivo");
	Command::new("rustc")
			.arg(&file_name.replace(".lb", ".rs"))
	        .spawn()
	       	.expect("no rustc compiler");
	/*
    if keep_file == false {
    	fs::remove_file(&file_name.replace(".lb", ".rs")).expect("Falha ao apagar o arquivo")

    }

    Command::new("./".to_owned() + &file_name.replace(".lb", ".rs"))
        	.spawn()
       		.expect("no file");
    */
};	

criar()
}