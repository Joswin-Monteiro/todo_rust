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

    if args.len() < 2 {
        eprintln!("Not enough arguments");
        exit(1);
    }

    parse_config(&args);
}

fn display_help() {
    println!("Usage: todo [options...]");
}

fn db_connect() -> Connection {
    let conn = match Connection::open("my_database.db") {
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

    let todo_iter = match stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Error querying todos: {}", e);
            exit(1);
        }
    };

    // Print out the todos
    println!("Todo items:");
    let mut counter: u32 = 0;
    for todo in todo_iter {
        counter += 1;
        match todo {
            Ok((_, name)) => println!("{}: {}", counter, name),
            Err(e) => eprintln!("Error reading todo: {}", e),
        }
    }
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

fn remove_todo(id: &str) {
    let conn = db_connect();
    let sql = "DELETE FROM todo WHERE id = ?1";
    match conn.execute(sql, &[id]) {
        Ok(rows_deleted) => {
            if rows_deleted == 0 {
                println!("No todo found");
            } else {
                println!("Successfully deleted {} row(s).", rows_deleted);
            }
        }
        Err(e) => eprintln!("Error deleting row: {}", e),
    };
}

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
                    remove_todo(&arg);
                } else {
                    eprintln!("id of the todo argument needed!");
                }
            }
            "show" => {
                display_todo_list();
            }
            _ => {
                display_help();
                exit(1);
            }
        }
    }
}
