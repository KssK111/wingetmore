use std::env::args;
use tokio::process::Command;
pub enum Mode {
    Install,
    Uninstall,
    Upgrade,
    Search,
    Other,
}

pub fn parse_args() -> [Vec<String>; 5] {
    let mut vec_array: [Vec<String>; 5] =
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut mode = Mode::Other;

    for arg in args().skip(1) {
        if arg == "install" {
            mode = Mode::Install;
        } else if arg == "uninstall" {
            mode = Mode::Uninstall;
        } else if arg == "upgrade" {
            mode = Mode::Upgrade;
        } else if arg == "search" {
            mode = Mode::Search;
        } else if arg == "other" {
            mode = Mode::Other;
        } else {
            vec_array[match mode {
                Mode::Install => 0,
                Mode::Uninstall => 1,
                Mode::Upgrade => 2,
                Mode::Search => 3,
                Mode::Other => 4,
            }]
            .push(arg);
        }
    }

    vec_array
}

pub async fn make_future(argument: String, mode: Mode) {
    let args_to_print = match mode {
        Mode::Install => format!("install {}", argument),
        Mode::Uninstall => format!("uninstall {}", argument),
        Mode::Upgrade => format!("upgrade {}", argument),
        Mode::Search => format!("search {}", argument),
        Mode::Other => argument,
    };
    let command_args = args_to_print.split_whitespace();
    let command_args = {
        let mut temp = Vec::new();
        for arg in command_args {
            temp.push(arg);
        }
        temp
    };
    println!("Trying to {}", args_to_print);

    let mut command = Command::new("winget.exe");
    command.args(command_args);

    let command_result = command.output().await;
    match command_result {
        Ok(output) => {
            println!("OK - {} has ended", args_to_print);
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(error) => {
            eprintln!("ERROR - {} failed\n{}", args_to_print, error);
        }
    }
}

pub fn get_update_names(output: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut found_header = false;
    let mut in_updates_section = false;

    for line in output.lines() {
        if line.contains("Name") && line.contains("Id") && line.contains("Version") {
            found_header = true;
            in_updates_section = true;
            continue;
        }

        if found_header && line.contains("----") {
            continue;
        }

        if line.contains("upgrades available")
            || line.contains("No package found matching input criteria")
        {
            break;
        }

        if in_updates_section && !line.trim().is_empty() {
            let columns: Vec<&str> = line.split_whitespace().collect();

            if columns.len() >= 4 {
                let name = columns[columns.len() - 3];
                names.push(name.to_string());
            }
        }
    }
    names
}

pub fn help() {
    println!("How to use this program:");
    println!(
        "wgm install program1 program2 program3 uninstall ... upgrade ... search ... other \"full command\""
    );

    println!("\nArguments:");
    println!("\tinstall");
    println!("\tuninstall");
    println!("\tupgrade (handles --all)");
    println!("\tsearch");
    println!("\tother");

    println!("\nAn example:");
    println!(
        "wgm upgrade --all search Microsoft. Google. Github other \"show Brave.Brave.Beta --versions\" search Azure"
    );
}
