use std::env;

fn about() {
    println!("Rust Snitch by Kokovin Aleksei");
    println!("I will hopefully write about the args here");
    println!("I'm sorry if I didn't");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("I expected some args!");
        about();
    }
}
