use std::io::{self, Write};

fn main() {
    let input = get_user_input();
    let input_array = input_to_array(&input);
    println!("{:?}", input_array);
}

#[allow(unused_must_use)]
fn get_user_input() -> String {
    print!("IPv4 Address: ");
    io::stdout().flush();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => input
    }
}

fn input_to_array (input: &str) -> Vec<&str> {
    input.trim().split('.').collect()
}