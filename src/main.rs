use rusqlite::Connection;
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

fn display_help() {
    println!("Usage: todo [options...]");
}

fn db_connect() -> Connection {
    let conn = match Connection::open_in_memory() {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Error opening in database: {}", e);
            exit(1);
        }
    };

    match conn.execute("CREATE TABLE IF NOT EXISTS todo(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);", ()){
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error creating table: {}", e);
            exit(1);
        }
    };

    conn
}

fn display_todo_list() {
    let conn = db_connect();

    let mut stmt = match conn.prepare("SELECT id, name FROM todo") {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("Error preparing statement: {}", e);
            exit(1);
        }
    };

    //     let todo_iter() = match stmt.query_map([], |row|{
    //         Ok()
    //     });
}

fn add_todo(name: &str) {
    let conn = db_connect();

    match conn.execute(
        "
        INSERT INTO todo (name) VALUES (?1)
        ",
        [name],
    ) {
        Ok(_) => {
            println!("Todo item added: {}", name);
        }
        Err(e) => {
            eprintln!("Error adding todo item: {}", e);
            exit(1);
        }
    }
}

// fn remove_todo(){}

fn parse_config(args: &[String]) {
    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "add" => {
                println!("add argument");
                if let Some(arg) = iter.next() {
                    //println!("name of the todo: {}", arg);
                    add_todo(&arg);
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
            _ => {
                display_help();
                exit(1);
            }
        }
    }
}
