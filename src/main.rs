use hex_util::command;
use hex_util::hexlib::HexLib;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Get command line arguments
    let tokens = env::args().skip(1).collect();
    let options = command::parse_command_line(&tokens);
    if options.is_empty() {
        println!("Usage: {} <option name>", &tokens[0]);
        return ExitCode::FAILURE;
    }

    let mut hex = HexLib::new();

    // Analyze & Execute command options
    match command::analyze_option(options) {
        Ok(list) => {
            for (opt_info, opt_args) in list {
                if let Err(e) = (opt_info.callback)(opt_args, &mut hex) {
                    eprintln!("Error: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
