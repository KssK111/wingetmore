use futures::{StreamExt, stream::FuturesUnordered};
use std::{borrow::Cow, process::exit};
use tokio::process::Command;
use wingetmore::*;

#[tokio::main]
async fn main() {
    let mut tasks = FuturesUnordered::new();
    let [
        install_vec,
        uninstall_vec,
        mut upgrade_vec,
        search_vec,
        other_vec,
    ] = parse_args();
    if install_vec.is_empty()
        && uninstall_vec.is_empty()
        && upgrade_vec.is_empty()
        && search_vec.is_empty()
        && (other_vec.is_empty()
            || other_vec.contains(&"--help".to_string())
            || other_vec.contains(&"/help".to_string())
            || other_vec.contains(&"help".to_string()))
    {
        help();
        exit(0);
    }

    let contains_all = upgrade_vec.contains(&"--all".to_string());
    upgrade_vec = if contains_all {
        let mut winget_upgrade = Command::new("winget.exe");
        winget_upgrade.arg("upgrade");
        let output = winget_upgrade.output().await;

        match output {
            Ok(output) => {
                let output = output.stdout;
                let output = String::from_utf8_lossy(&output);
                match output {
                    Cow::Borrowed(output) => get_update_names(output),
                    Cow::Owned(output) => get_update_names(output.as_str()),
                }
            }
            Err(error) => {
                eprintln!("ERR - {}", error);
                Vec::new()
            }
        }
    } else {
        upgrade_vec
    };

    for argument in install_vec {
        tasks.push(make_future(argument, Mode::Install));
    }
    for argument in uninstall_vec {
        tasks.push(make_future(argument, Mode::Uninstall));
    }
    for argument in upgrade_vec {
        tasks.push(make_future(argument, Mode::Upgrade));
    }
    for argument in search_vec {
        tasks.push(make_future(argument, Mode::Search));
    }
    for argument in other_vec {
        tasks.push(make_future(argument, Mode::Other));
    }
    while tasks.next().await.is_some() {}

    if contains_all {
        println!("Checking for remaining updates");
        make_future("--all".to_string(), Mode::Upgrade).await;
    }
}
