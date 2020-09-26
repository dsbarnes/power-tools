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

    for line in BufReader::new(yaml_file).lines().enumerate() {
        if let Some('#') = line.1.as_ref().unwrap().chars().next() {
            println!("This is a comment!");
            sed
            .arg(format!("-i '.yaml' {}s/^.{2}//g"), line.0)
            // .arg(line.unwrap())
            .status()
            .expect("Could not uncomment the line");
        }
        else {
            println!("This is NOT a comment!");
            sed
            .arg("-i '.yaml' $'i\\\n# '")
            // .arg(line.unwrap())
            .status()
            .expect("Could not comment the line");
        }
    }
    Ok(())
}
