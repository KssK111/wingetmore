use std::env::args;

use tokio::io::{self, stdin, AsyncBufReadExt, BufReader};

pub fn parse_args() -> [Vec<String>; 4]
{
    let mut vec_array: [Vec<String>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let (mut install,mut uninstall,mut upgrade/*,mut search*/) = (false, false, false/*, false*/);

    for arg in args().skip(1)
    {
        if arg == "install" {install = true; uninstall = false; upgrade = false;/* search = false;*/ continue;}
        if arg == "uninstall" {install = false; uninstall = true; upgrade = false;/* search = false;*/ continue;}
        if arg == "upgrade" {install = false; uninstall = false; upgrade = true;/* search = false;*/ continue;}
        if arg == "search" {install = false; uninstall = false; upgrade = false;/* search = true;*/ continue;}

        vec_array
        [
            if install {0}
            else if uninstall {1}
            else if upgrade {2}
            else {3}
        ].push(arg);
    }

    vec_array
}

pub mod make_future
{
    use tokio::process::Command;

    pub async fn install(argument: String)
    {
        println!("Trying to install: {}", argument);

        let mut command = Command::new("winget.exe");
        command.arg("install");
        command.arg(&argument);

        let command_result = command.output().await;
        match command_result
        {
            Ok(output) =>
            {
                println!("OK - install {} has ended", argument);
                if !output.stdout.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(error) =>
            {
                eprintln!("ERROR - install {} failed\n{}", argument, error);
            }
        }
    }

    pub async fn uninstall(argument: String)
    {
        println!("Trying to uninstall: {}", argument);

        let mut command = Command::new("winget.exe");
        command.arg("uninstall");
        command.arg(&argument);

        let command_result = command.output().await;
        match command_result
        {
            Ok(output) =>
            {
                println!("OK - uninstall {} has ended", argument);
                if !output.stdout.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(error) =>
            {
                eprintln!("ERROR - uninstall {} failed\n{}", argument, error);
            }
        }
    }

    pub async fn upgrade(argument: String)
    {
        println!("Trying to upgrade: {}", argument);

        let mut command = Command::new("winget.exe");
        command.arg("upgrade");
        command.arg(&argument);

        let command_result = command.output().await;
        match command_result
        {
            Ok(output) =>
            {
                println!("OK - upgrade {} has ended", argument);
                if !output.stdout.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(error) =>
            {
                eprintln!("ERROR - upgrade {} failed\n{}", argument, error);
            }
        }
    }

    pub async fn search(argument: String)
    {
        println!("Trying to search: {}", argument);

        let mut command = Command::new("winget.exe");
        command.arg("search");
        command.arg(&argument);

        let command_result = command.output().await;
        match command_result
        {
            Ok(output) =>
            {
                println!("OK - search {} has ended", argument);
                if !output.stdout.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty()
                {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(error) =>
            {
                eprintln!("ERROR - search {} failed\n{}", argument, error);
            }
        }
    }
}

pub async fn take_input() -> Result<String, io::Error>
{
    let mut buffer = String::new();
    let mut buf_reader = BufReader::new(stdin());

    let result = buf_reader.read_line(&mut buffer).await;
    match result
    {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error)
    }
}

pub fn get_update_names(output: &str) -> Vec<String>
{
    let mut names = Vec::new();
    let mut found_header = false;
    let mut in_updates_section = false;

    for line in output.lines()
    {
        if line.contains("Name") && line.contains("Id") && line.contains("Version")
        {
            found_header = true;
            in_updates_section = true;
            continue;
        }

        if found_header && line.contains("----")
        {
            continue;
        }

        if line.contains("upgrades available") || line.contains("No package found matching input criteria")
        {
            break;
        }

        if in_updates_section && !line.trim().is_empty()
        {
            let columns: Vec<&str> =
            line
                .split_whitespace()
                .collect();
            
            if columns.len() >= 4
            {
                let name = columns[columns.len() - 3];
                names.push(name.to_string());
            }
        }
    }
    names
}