use std::env;
use std::char;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || (&args[1]).len() == 0 {
        println!("Usage: ./rust-rot47 <text_to_encode/decode>");
        return;
    }

    let text = &args[1];
    let mut translation = String::new();

    for letter in text.chars() {
        let code_point = letter as u32;

        if code_point >= 33 && code_point <= 126 {
            let c = char::from_u32(33 + ((code_point + 14) % 94));
            translation.push(c.unwrap());
        } else {
            translation.push(letter);
        }
    }

    println!("{}", translation);
}
