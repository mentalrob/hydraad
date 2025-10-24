use std::io::{stdout, Write};

use clap::CommandFactory;
use crossterm::{cursor::{Hide, MoveTo}, terminal::{Clear, ClearType}, QueueableCommand};

pub const BANNER: &str = r#"                                                                                                    
 _   _           _                _    ____  
| | | |_   _  __| |_ __ __ _     / \  |  _ \ 
| |_| | | | |/ _` | '__/ _` |   / _ \ | | | |
|  _  | |_| | (_| | | | (_| |  / ___ \| |_| |
|_| |_|\__, |\__,_|_|  \__,_| /_/   \_\____/ 
       |___/                                                        

Made by mentalrob & baymorningstar.
"#;

pub fn clear_screen() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
    print_banner();
}

pub fn print_banner() {
    println!("{}", BANNER);
}

fn walk_commands(cmd: &clap::Command, prefix: String, mut list: Vec<String>) -> Vec<String> {
    let name = cmd.get_name();
    let full = if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{} {}", prefix, name)
    };

    for sub in cmd.get_subcommands() {
        list = walk_commands(sub, full.clone(), list);
    }
    list.push(full);
    list
}

pub fn list_all_commands<Clap: CommandFactory>() -> Vec<String> {
    let root = Clap::command();
    
    // Pass root
    let mut list = Vec::new();
    for sub in root.get_subcommands() {
        list = walk_commands(sub, String::new(), list);
    }
    list
}