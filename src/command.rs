use crate::hexlib::{range, utility, HexLib};
use indexmap::IndexMap;

type OptionFunc = fn(args: Vec<String>, hex_lib: &mut HexLib) -> Result<(), String>;

#[derive(Clone)]
pub struct CommandOptionInfo {
    pub callback: OptionFunc,
    description: String,
    priority: u32,
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

/////////////////////////////////////////////////////////////////////////

fn help_option(_args: Vec<String>, _hexlib: &mut HexLib) -> Result<(), String> {
    println!("Available options:");
    for (optname, params) in get_option_info_list() {
        println!(" - {} \t\t: {}", optname, params.description);
    }
    Ok(())
}

fn version_option(_args: Vec<String>, _hexlib: &mut HexLib) -> Result<(), String> {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn range_option(args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
    if args.len() != 2 {
        return Err("Specify <start> <end>".to_string());
    }

    let mut start_addr: u32 = 0;
    let mut end_addr: u32 = 0;

    if let Some(start_hex) = args[0].strip_prefix("0x") {
        match u32::from_str_radix(start_hex, 16) {
            Ok(val) => start_addr = val,
            Err(_) => return Err("Invalid start address format".to_string()),
        }
    }

    if let Some(end_hex) = args[1].strip_prefix("0x") {
        match u32::from_str_radix(end_hex, 16) {
            Ok(val) => end_addr = val,
            Err(_) => return Err("Invalid end address format".to_string()),
        }
    }

    utility::rebuild_ranges(&mut hexlib.user_ranges, range::AddressRange::new(start_addr, end_addr));
    if !hexlib.user_ranges.is_empty() {
        hexlib.has_user_ranges = true;
    }

    Ok(())
}

fn view_option(args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
    // single argument <filename>
    if args.len() != 1 {
        return Err("Specify <filename>".to_string());
    }

    let filename = &args[0];
    if let Err(e) = hexlib.read_file(filename) {
        return Err(format!("Failed to read file: {}", e));
    }

    if hexlib.has_user_ranges {
        for range in &hexlib.user_ranges {
            let start = range.start;
            let end = range.end;
            hexlib.show_hex_data(start, end);
        }
    } else {
        for range in hexlib.hex_data.get_data_ranges() {
            let start = range.start;
            let end = range.end;
            hexlib.show_hex_data(start, end);
        }
    }

    Ok(())
}

fn create_option(args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
    Ok(())
}

fn debug_option(_args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
    for range in &hexlib.user_ranges {
        println!("Range: 0x{:08X} - 0x{:08X}", range.start, range.end);
    }

    Ok(())
}

fn get_option_info_list() -> IndexMap<String, CommandOptionInfo> {
    vec![
        (
            "help".to_owned(),
            CommandOptionInfo {
                callback: help_option,
                description: "Show help message".to_string(),
                priority: 1,
            },
        ),
        (
            "version".to_owned(),
            CommandOptionInfo {
                callback: version_option,
                description: "Show version".to_string(),
                priority: 1,
            },
        ),
        (
            "range".to_owned(),
            CommandOptionInfo {
                callback: range_option,
                description: "Specify address range".to_string(),
                priority: 2,
            },
        ),
        (
            "view".to_owned(),
            CommandOptionInfo {
                callback: view_option,
                description: "View data in hex file".to_string(),
                priority: 3,
            },
        ),
        (
            "create".to_owned(),
            CommandOptionInfo {
                callback: create_option,
                description: "Create hex file".to_string(),
                priority: 4,
            },
        ),
        (
            "debug".to_owned(),
            CommandOptionInfo {
                callback: debug_option,
                description: "Debug option".to_string(),
                priority: 99,
            },
        ),
    ]
    .into_iter()
    .collect::<IndexMap<_, _>>()
}

// Analyze and sort options based on priority
pub fn analyze_option(options: Vec<(String, Vec<String>)>) -> Result<Vec<(CommandOptionInfo, Vec<String>)>, String> {
    let mut use_options = Vec::new();
    let option_list = get_option_info_list();

    for (opt_name, opt_args) in options {
        if let Some(opt_info) = option_list.get(&opt_name) {
            use_options.push((opt_info.clone(), opt_args.clone()));
        } else {
            return Err(format!("Unknown option: --{}", opt_name));
        }
    }

    use_options.sort_by_key(|(info, _)| info.priority);

    Ok(use_options)
}
