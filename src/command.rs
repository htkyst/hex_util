use crate::hexlib::{data_type::range::AddressRange, utility::range_utils};
use crate::hexlib::HexLib;
use std::collections::HashMap;

enum ConvertFormat {
    ToIntelHex,
    ToMotorolaSRecord,
    ToRawBinary,
}

#[derive(Clone)]
struct OptionInfo {
    name: String,
    description: String,
    priority: u32,
}

type OptionFunc = fn(&mut Command, Vec<String>, &mut HexLib) -> Result<(), String>;

pub struct Command {
    run_options: Vec<(OptionFunc, Vec<String>)>,
    ranges: Vec<AddressRange>,
}

impl Command {
    const OPTION_HELP: &str = "help";
    const OPTION_VERSION: &str = "version";
    const OPTION_RANGE: &str = "range";
    const OPTION_VIEW: &str = "view";
    const OPTION_CONVERT: &str = "convert";
    const OPTION_CREATE: &str = "create";
    const OPTION_DEBUG: &str = "debug";

    /**
     * * Create a new Command instance
     */
    pub fn new() -> Command {
        Command {
            run_options: Vec::new(),
            ranges: Vec::new(),
        }
    }

    fn get_option_list(&self) -> HashMap<String, OptionInfo> {
        vec![
            (
                Self::OPTION_HELP.to_owned(),
                OptionInfo {
                    name: Self::OPTION_HELP.to_string(),
                    description: "Show help message".to_string(),
                    priority: 1,
                },
            ),
            (
                Self::OPTION_VERSION.to_owned(),
                OptionInfo {
                    name: Self::OPTION_VERSION.to_string(),
                    description: "Show version".to_string(),
                    priority: 1,
                },
            ),
            (
                Self::OPTION_RANGE.to_owned(),
                OptionInfo {
                    name: Self::OPTION_RANGE.to_string(),
                    description: "Specify address range".to_string(),
                    priority: 2,
                },
            ),
            (
                Self::OPTION_VIEW.to_owned(),
                OptionInfo {
                    name: Self::OPTION_VIEW.to_string(),
                    description: "View data in hex file".to_string(),
                    priority: 3,
                },
            ),
            (
                Self::OPTION_CONVERT.to_owned(),
                OptionInfo {
                    name: Self::OPTION_CONVERT.to_string(),
                    description: "Convert hex file format".to_string(),
                    priority: 4,
                },
            ),
            (
                Self::OPTION_CREATE.to_owned(),
                OptionInfo {
                    name: Self::OPTION_CREATE.to_string(),
                    description: "Create hex file".to_string(),
                    priority: 4,
                },
            ),
            (
                Self::OPTION_DEBUG.to_owned(),
                OptionInfo {
                    name: Self::OPTION_DEBUG.to_string(),
                    description: "Debug option".to_string(),
                    priority: 99,
                },
            ),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    }

    // Analyze command line options
    pub fn analyze(&mut self, opt_name_args: Vec<(String, Vec<String>)>) -> Result<(), String> {
        let option_list = self.get_option_list();

        let mut specific_options = Vec::new();
        for (opt_name, opt_args) in opt_name_args {
            if let Some(opt_info) = option_list.get(&opt_name) {
                specific_options.push((opt_info.clone(), opt_args.clone()));
            } else {
                return Err(format!("Unknown option: --{}", opt_name));
            }
        }

        specific_options.sort_by_key(|(info, _)| info.priority);

        for (opt_info, opt_args) in specific_options {
            if let Some(callback) = self.get_option_callback(&opt_info.name) {
                self.run_options.push((callback, opt_args));
            } else {
                return Err(format!("No callback found for option: --{}", opt_info.name));
            }
        }

        Ok(())
    }

    // Run the analyzed commands
    pub fn run(&mut self, hexlib: &mut HexLib) -> Result<(), String> {
        let options: Vec<_> = self.run_options.iter().cloned().collect();
        for (opt_func, opt_args) in options {
            match (opt_func)(self, opt_args, hexlib) {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    //// Command Option Implementations ////

    fn help_option(&mut self, _args: Vec<String>, _hexlib: &mut HexLib) -> Result<(), String> {
        println!("Available options:");
        for (optname, params) in self.get_option_list() {
            println!(" - {} \t\t: {}", optname, params.description);
        }
        Ok(())
    }

    fn version_option(&mut self, _args: Vec<String>, _hexlib: &mut HexLib) -> Result<(), String> {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }

    fn range_option(&mut self, args: Vec<String>, _hexlib: &mut HexLib) -> Result<(), String> {
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

        range_utils::rebuild_ranges(&mut self.ranges, AddressRange::new(start_addr, end_addr));

        Ok(())
    }

    fn view_option(&mut self, args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
        // single argument <filename>
        if args.len() != 1 {
            return Err("Specify <filename>".to_string());
        }

        let filename = &args[0];
        if let Err(e) = hexlib.read_file(filename) {
            return Err(format!("Failed to read file: {}", e));
        }

        if self.ranges.len() > 0 {
            for range in &self.ranges {
                let start = range.start;
                let end = range.end;
                hexlib.show_hex_data(start, end);
            }
        } else {
            for range in hexlib.data_buffer.get_data_ranges() {
                let start = range.start;
                let end = range.end;
                hexlib.show_hex_data(start, end);
            }
        }

        Ok(())
    }

    fn convert_option(&mut self, args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
        if args.len() != 2 {
            return Err("Specify <input filename> <format>".to_string());
        }

        let filename = &args[0];
        let format = &args[1];

        if let Err(e) = hexlib.read_file(filename) {
            return Err(format!("Failed to read file: {}", e));
        }

        Ok(())
    }

    fn create_option(&mut self, args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
        Ok(())
    }

    fn debug_option(&mut self, _args: Vec<String>, hexlib: &mut HexLib) -> Result<(), String> {
        println!("Debug Info:");

        for range in self.ranges.iter() {
            println!(" Range: 0x{:08X} - 0x{:08X}", range.start, range.end);
        }
        Ok(())
    }

    fn get_option_callback(&self, option_name: &str) -> Option<OptionFunc> {
        match option_name {
            Self::OPTION_HELP => Some(Command::help_option),
            Self::OPTION_VERSION => Some(Command::version_option),
            Self::OPTION_RANGE => Some(Command::range_option),
            Self::OPTION_VIEW => Some(Command::view_option),
            Self::OPTION_CONVERT => Some(Command::convert_option),
            Self::OPTION_CREATE => Some(Command::create_option),
            Self::OPTION_DEBUG => Some(Command::debug_option),
            _ => None,
        }
    }
}
