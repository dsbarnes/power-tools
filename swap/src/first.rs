use std::process::Command;
use std::str;
use clap::App;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), &'static str> {
    if cfg!(target_os = "windows") {
        return 
        Err("This program does not work for windows, and I don't plan to port it") }

    let args = App::new("comment-swap")
        .args_from_usage(
            "<FILE>     'The file to swap'"
        ).get_matches();

    let yaml_path = args.value_of("FILE")
        .expect("Could not retrieve the path");
    let yaml_file = File::open(yaml_path)
        .expect("Could not open the yaml_path");

    let mut sed = Command::new("sed");
    let mut mv = Command::new("mv");

    for (ln, line) in BufReader::new(yaml_file).lines().enumerate() {
        if let Some('#') = line
            .as_ref()
            .expect("could not match the line ref")
            .chars()
            .next()
        {
            let cmd = format!("s/../*/ {}", 
                shellexpand::tilde("~/Documents/power-tools/comments.yaml"));

            sed.arg(cmd).status()
                .expect("Could not do the sed stuff");
        }
    }

    Ok(())
}
