std::io::{stdin,stdout,Write};fn pergunte(conteudo: &str) -> String{
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
}macro_rules! diga {
    () => {
        println!();
    };

    (? $head:expr, $($tail:tt)*) => {
        print!("{:?} ", $head);    
        python_print!($($tail)*);  
    };

    (? $head:expr) => {
        println!("{:?}", $head);
    };

    ($head:expr, $($tail:tt)*) => {
        print!("{} ", $head);      
        python_print!($($tail)*);  
    };

    ($head:expr) => {
        println!("{}", $head);
    };
}


fn main() {
função Teste(texto: &str) -> RetType {
    let mut state = [[0u8; 16];16];
    draw(&state);
    diga!(texto);
}
let response = pergunte("oi")
diga!("irei rodar uma função!");
Teste(response);

}