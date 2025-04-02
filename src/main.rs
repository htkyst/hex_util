mod range;

use std::env;
use range::Range;

struct OptionInfo {
    opt: String,
    args: Vec<String>,
}

enum HexFormatType {
    None,
    IntelHex,
    SRecord,
    Binary,
}

fn convert_hex_format_to_enum(hex_format: &str) -> HexFormatType {
    match hex_format {
        "hex" => HexFormatType::IntelHex,
        "mot" => HexFormatType::SRecord,
        "bin" => HexFormatType::Binary,
        _ => HexFormatType::None,
    }
}

enum HexDataType {
    None,
    Random,
    Incremental,
    Fill,
}

fn convert_hex_data_type_to_enum(hex_file_type: &str) -> HexDataType {
    match hex_file_type {
        "rand" => HexDataType::Random,
        "inc" => HexDataType::Incremental,
        "fill" => HexDataType::Fill,
        _ => HexDataType::None,
    }
}

struct OptionData {
    range: Vec<Range<u32>>,
}

fn check_option_format(option: &str) -> bool {
    option.len() > 2 && option[0..2].eq("--")
}

fn parse_command_line(command_line: Vec<String>) -> Vec<OptionInfo> {
    let mut options: Vec<OptionInfo> = Vec::new();
    let mut iter = command_line.into_iter().peekable();

    while let Some(token) = iter.next() {
        if check_option_format(&token) {
            let mut args = Vec::new();
            let opt_name = token.trim_start_matches("--").to_string();

            while let Some(arg) = iter.peek() {
                if check_option_format(arg) {
                    break;
                }
                args.push(iter.next().unwrap());
            }

            options.push(OptionInfo {
                opt: opt_name,
                args,
            });
        }
    }

    return options;
}

fn convert_hex_string_to_u32(hex_str: &str) -> Result<u32, String> {
    match u32::from_str_radix(&hex_str[2..], 16) {
        Ok(value) => Ok(value),
        Err(_) => Err("Invalid hex string".to_string()),
    }
}

fn check_sub_options(options: &Vec<OptionInfo>, option_data: &mut OptionData) -> Result<(), String> {
    for option in options {
        match option.opt.as_str() {
            "range" => {
                let range_num = option.args.len() / 2;
                if range_num % 2 != 0 {
                    return Err("Need even number of arguments for range option".to_string());
                }

                for r in 0..range_num {
                    let start_addr = convert_hex_string_to_u32(&option.args[r * 2]);
                    let end_addr = convert_hex_string_to_u32(&option.args[r * 2 + 1]);

                    match (start_addr, end_addr) {
                        (Ok(start), Ok(end)) => {
                            option_data.range.push(Range::new(start, end));
                        }
                        _ => {
                            return Err("Invalid address format".to_string());
                        }
                    }
                }
            }
            _ => {
                return Err(format!("Unknown sub-option: {}", option.opt.as_str()));
            }
        }
    }

    Ok(())
}

fn check_file_format(file_name: &str) -> Result<HexFormatType, String> {
    let file_extension = file_name.split('.').last().unwrap_or("");
    match file_extension {
        "hex" => Ok(HexFormatType::IntelHex),
        "mot" => Ok(HexFormatType::SRecord),
        "bin" => Ok(HexFormatType::Binary),
        _ => Err("Unknown file format".to_string()),
    }
}

fn execute_show_command(option_data: &OptionData, filename: String) -> Result<(), String> {
    let file_format = check_file_format(&filename)?;
    

    Ok(())
}

fn execute_convert_command(option_data: &OptionData, filename: String, format: HexFormatType) -> Result<(), String> {

    Ok(())
}

fn execute_create_command(option_data: &OptionData, filename: String, size: u32, mode: HexDataType) -> Result<(), String> {

    Ok(())
}

fn execute_remove_command(option_data: &OptionData, filename: String) -> Result<(), String> {

    Ok(())
}

fn analyze_command_line(options: &Vec<OptionInfo>) -> Result<(), String> {
    let option_data = &mut OptionData {
        range: Vec::new(),
    };

    if let Err(err) = check_sub_options(options, option_data) {
        return Err(err);
    }

    for option in options {
        match option.opt.as_str() {
            "show" => {
                if option.args.len() != 1 {
                    return Err("Need one argument for show option".to_string());
                }


            }
            "convert" => {
                if option.args.len() != 2 {
                    return Err("Need two arguments for convert option".to_string());
                }

            }
            "create" => {
                if option.args.len() != 3 {
                    return Err("Need two arguments for create option".to_string());
                }

            }
            "remove" => {
                if option.args.len() != 1 {
                    return Err("Need one argument for remove option".to_string());
                }
            }
            _ => {
                println!("Unknown option: {}", option.opt);
            }
        }
    }

    Ok(())
}

fn main() {
    let command_line: Vec<String> = env::args().collect();
    if command_line.len() < 2 {
        println!("Usage: {} <command>", command_line[0]);
        return;
    }

    let options = parse_command_line(command_line);
    if let Err(err) = analyze_command_line(&options) {
        println!("Error: {}", err);
        return;
    }
}
