use hex_util::command;
use hex_util::hexlib::HexLib;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Get command line arguments
    let args = env::args().skip(1).collect();
    let tokens = parse_command_line(&args);
    if tokens.is_empty() {
        println!("Usage: {} <option name>", &args[0]);
        return ExitCode::FAILURE;
    }

    let mut command = command::Command::new();
    let mut hex = HexLib::new();

    // Analyze & Execute command options
    match command.analyze(tokens) {
        Ok(()) => {
            if let Err(e) = command.run(&mut hex) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

// Function to check if a string is an option format
// Example: Strings like "--aaa" or "--ccc" (at least 2 characters, starting with "--") are considered options
fn check_option_format(option: &str) -> bool {
    option.len() > 2 && option[0..2].eq("--")
}

// Parse command line arguments into a list of options and their arguments
pub fn parse_command_line(tokens: &Vec<String>) -> Vec<(String, Vec<String>)> {
    if tokens.len() < 1 {
        return Vec::new();
    }

    let mut options = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        if check_option_format(&token) {
            let mut args = Vec::new();
            let opt_name = token.trim_start_matches("--").to_string();

            while let Some(arg) = iter.peek() {
                if check_option_format(arg) {
                    break;
                }
                args.push(iter.next().unwrap().to_string());
            }

            options.push((opt_name, args));
        }
    }

    options
}
