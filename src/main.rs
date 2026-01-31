use hex_util::command;
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

    // Analyze & Execute command options
    let mut command = command::Command::new();
    match command.analyze(tokens) {
        Ok(()) => {
            if let Err(e) = command.run() {
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
/// 
/// # Returns
/// 
/// A vector of tuples, where each tuple contains an option name (without the leading "--")
/// and a vector of its associated arguments.
/// 
/// # Examples
/// 
/// ```
/// let args = vec!["--input".to_string(), "file.hex".to_string(), "--output".to_string(), "out.bin".to_string()];
/// let result = parse_command_line(&args);
/// // result: [("input", ["file.hex"]), ("output", ["out.bin"])]
/// `
fn parse_command_line(tokens: &Vec<String>) -> Vec<(String, Vec<String>)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_option_format() {
        assert!(check_option_format("--option") == true);
        assert!(check_option_format("-option") == false);
        assert!(check_option_format("option") == false);
        assert!(check_option_format("--") == false);
    }

    #[test]
    fn test_parse_command_line() {
        let args = vec![
            "--opt1".to_string(),
            "p1".to_string(),
            "--opt2".to_string(),
            "p1".to_string(),
            "p2".to_string(),
            "p3".to_string(),
        ];
        let result = parse_command_line(&args);
        assert_eq!(
            result,
            vec![
                ("opt1".to_string(), vec!["p1".to_string()]),
                ("opt2".to_string(), vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]),
            ]
        );

        let args_no_options = vec!["file.hex".to_string(), "out.bin".to_string()];
        let result_no_options = parse_command_line(&args_no_options);
        assert!(result_no_options.is_empty());
    }
}