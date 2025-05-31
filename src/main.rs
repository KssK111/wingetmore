use std::{borrow::Cow, pin::Pin};
use std::future::Future;
use futures::{stream::FuturesUnordered, StreamExt};
use tokio::process::Command;
use wingetmore::*;

#[tokio::main]
async fn main()
{
    let mut tasks = FuturesUnordered::new();
    let [install_vec, uninstall_vec, mut upgrade_vec, search_vec] = parse_args();

    upgrade_vec =
    if upgrade_vec.contains(&"--all".to_string())
    {
        let mut winget_upgrade = Command::new("winget.exe");
        winget_upgrade.arg("upgrade");
        let output = winget_upgrade.output().await;

        match output
        {
            Ok(output) =>
            {
                let output = output.stdout;
                let output = String::from_utf8_lossy(&output);
                match output
                {
                    Cow::Borrowed(output) => get_update_names(output),
                    Cow::Owned(output) => get_update_names(output.as_str())
                }
            }
            Err(error) =>
            {
                eprintln!("ERR - {}", error);
                Vec::new()
            }
        }
    }
    else {upgrade_vec};

    let operations: [(Vec<String>, fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>>); 4] =
    [
        (install_vec, |arg| Box::pin(make_future::install(arg))),
        (uninstall_vec, |arg| Box::pin(make_future::uninstall(arg))),
        (upgrade_vec, |arg| Box::pin(make_future::upgrade(arg))),
        (search_vec, |arg| Box::pin(make_future::search(arg))),
    ];

    for (args, fut_function) in operations.iter()
    {
        for argument in args.iter()
        {
            tasks.push(fut_function(argument.to_owned()));
        }
    }


    while let Some(_) = tasks.next().await
    {
    }
}