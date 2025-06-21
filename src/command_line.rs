use std::env;

pub struct OptionInfo {
    pub opt: String,
    pub args: Vec<String>,
}

pub fn get_command_line() -> Vec<String> {
    env::args().skip(1).collect()
}

// オプション形式かどうかを判定する関数
// 例: "--aaa" や "--ccc" のような2文字以上で "--" から始まる文字列をオプションとみなす
fn check_option_format(option: &str) -> bool {
    option.len() > 2 && option[0..2].eq("--")
}

pub fn parse_command_line(command_line: Vec<String>) -> Vec<OptionInfo> {
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

pub fn show_command_line_options(options: &Vec<OptionInfo>) {
    for opt in options {
        if opt.args.is_empty() {
            println!("{} -> (no arguments)", opt.opt);
        } else {
            println!("{} -> [{}]", opt.opt, opt.args.join(", "));
        }
    }
}