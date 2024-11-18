use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments");
    } else {
        for arg in args.iter() {
            println!("{}", arg);
        }
    }
}
