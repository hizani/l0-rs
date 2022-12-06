use std::io::Write;
use std::process::ExitCode;
use std::{env, fs};

use redis::Commands;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: pub FILE").unwrap();
        return ExitCode::FAILURE;
    }

    let client = redis::Client::open("redis://127.0.0.1/").expect("Broken redis URL");
    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            return ExitCode::FAILURE;
        }
    };

    let contents = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            return ExitCode::FAILURE;
        }
    };
    let contents = contents.replace("\n", "").replace(" ", "");

    let delivery_count: i32 = match con.publish("chan", contents) {
        Ok(n) => n,
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            return ExitCode::FAILURE;
        }
    };

    println!("delivered\t{} times", delivery_count);
    ExitCode::SUCCESS
}
