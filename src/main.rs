use std::env;
use std::char;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1].len() == 0 {
        println!("Usage: ./rust-rot47 <encode/decode text>");
        return;
    }

    println!("{}", (&args[1]).chars().map(translate).collect::<String>());
}

fn translate(ch: char) -> char {
    let code_point = ch as u32;

    if code_point >= 33 && code_point <= 126 {
        return char::from_u32(33 + ((code_point + 14) % 94)).unwrap();
    }

    ch
}
