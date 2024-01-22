use std::{
    env,
    ffi::OsString,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

mod history;
use history::history::History;

fn main() {
    let history: History = History::new();

    ctrlc::set_handler(|| {
        println!();
        print!("kshell ➜ ");
        stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        print!("kshell ➜ ");
        stdout().flush().unwrap();

        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        history.store_history(&input);

        let mut parts = input.trim().split_whitespace();
        let command: &str = parts.next().unwrap();
        let args: Vec<OsString> = parts.into_iter().map(|arg| arg.into()).collect();

        if command == "\u{1b}[A" {
            let history_vec: Vec<String> = history.get_history();
            let history_len: usize = history_vec.len();
            if history_len == 0 {
                continue;
            }
            let last_command: String = history_vec[history_len - 1].clone();
            println!("{}", last_command);
            stdout().flush().unwrap();
            continue;
        }

        match command {
            "cd" => {
                let path: &str = args
                    .iter()
                    .peekable()
                    .peek()
                    .map_or("/", |x| x.to_str().unwrap_or("/"));
                let root = Path::new(path);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            commnd => {
                let mut child = Command::new(commnd)
                    .args(args)
                    .spawn()
                    .expect("Failed to execute command");
                child.wait().unwrap();
            }
        }
    }
}