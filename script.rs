use std::io::{stdin,stdout,Write};
fn pergunte(conteudo: &str) -> String{
	let mut variavel =String::new(); 
	print!("{}\n", conteudo);
	let _ =stdout().flush(); 
	let _ = stdin().read_line(&mut variavel);
	return variavel.replace("\n", "")
}
macro_rules! diga {
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

fn main() {
let mut state = [[0u8; 16];16];
    for y in 1..15{
    	for x in 1..15 {
    		state[y][x] = 136;
    	}
    }
    for y in 2..14{
    	for x in 2..14 {
    		state[y][x] = 130;
    	}
    }
    for x in 3..13 {
    	state[3][x] = 94;
    	state[6][x] = 94;
    	state[9][x] = 94;
    	state[12][x] = 94;
    }
    for y in 3..13 {
    	state[y][3] = 94;
    	state[y][6] = 94;
    	state[y][9] = 94;
    	state[y][12] = 94;
    }
    for xy in 1..4{
     	state[xy+11][xy] = 58;
     	state[xy][xy+11] = 58;
    }
    let mut pos = 3;
    for y in 1..4{
    	state[y][pos] = 58;
    	state[y+11][pos+11] = 58;
    	pos -=1
    }
	draw(&state);
}