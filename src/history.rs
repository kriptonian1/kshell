pub mod history {
    use std::env;
    use std::io::Write;

    pub struct History {
        history_file: String,
    }

    impl History {
        /// The function creates a new instance of the `History` struct with a specified history file
        /// path.
        ///
        /// Returns:
        ///
        /// an instance of the `History` struct.
        pub fn new() -> History {
            let home_dir: String = env::var("HOME").unwrap();
            let history_file: String = format!("{}/.kshell_history", home_dir);
            History { history_file }
        }

        /// The `store_history` function stores a command in a history file, excluding empty commands.
        ///
        /// Arguments:
        ///
        /// * `command`: The `command` parameter is a string that represents a command that needs to be stored
        /// in the history.
        pub fn store_history(&self, command: &str) {
            let command: String = command.trim().replace("\n", "");

            if command == "" || command == "\u{1b}[A" || command == "\u{1b}[B" {
                // Don't store empty commands
                return;
            }

            let mut history = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.history_file)
                .unwrap();
            writeln!(history, "{}", command).expect("Unable to write history"); // TODO: add unix timestamp
        }

		#[allow(dead_code)]
        pub fn get_history(&self) -> Vec<String> {
            match std::fs::read_to_string(&self.history_file) {
                Ok(contents) => contents.lines().map(|s| s.to_string()).collect(),
                Err(_) => Vec::new(),
            }
        }
    }
}
