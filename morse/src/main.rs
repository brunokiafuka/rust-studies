pub mod encoder;

use std::io;

fn main() {
    let mut text = String::new();

    println!("Please input to be encoded");
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    let encoded_morse_text = encoder::encode(&text);

    println!("Encoded --> {}", encoded_morse_text.unwrap());
}
