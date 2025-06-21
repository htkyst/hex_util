pub struct CommandInfo {
    pub optname: String,
    description: String,
    pub callback: fn(String, Vec<String>) -> Result<(), String>,
}

pub fn get_command_list() -> Vec<CommandInfo> {
    vec![
        CommandInfo {
            optname: "help".to_string(),
            description: "Show help message".to_string(),
            callback: help_command,
        },
        CommandInfo {
            optname: "version".to_string(),
            description: "Show version".to_string(),
            callback: version_command,
        },
        CommandInfo {
            optname: "list".to_string(),
            description: "Show command list".to_string(),
            callback: list_command,
        },
    ]
}

fn help_command(_optname: String, _args: Vec<String>) -> Result<(), String> {
    println!("Available commands:");
    for command in get_command_list() {
        println!(" - {} \t\t: {}", command.optname, command.description);
    }
    Ok(())
}

fn version_command(_optname: String, _args: Vec<String>) -> Result<(), String> {
    println!("Hex-util 1.0.0");
    Ok(())
}

fn list_command(_optname: String, _args: Vec<String>) -> Result<(), String> {
    println!("Listing items...");
    // Here you would normally list items from a data source
    Ok(())
}