use std::env;
use std::process::exit;

// struct Config {
//     add: bool,
//     todo_name: String,
//
//     done: bool,
//     todo_id: u8,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Not enough arguments");
        exit(1);
    }

    parse_config(&args);
}

fn parse_config(args: &[String]) {
    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "add" => {
                println!("add argument");
                if let Some(arg) = iter.next() {
                    println!("name of the todo: {}", arg);
                } else {
                    eprintln!("name of the add argument needed!");
                }
            }
            "done" => {
                println!("done argument");
                if let Some(arg) = iter.next() {
                    println!("id of the todo: {}", arg);
                } else {
                    eprintln!("id of the todo argument needed!");
                }
            }
            _ => println!("name of the todo"),
        }
    }
}
