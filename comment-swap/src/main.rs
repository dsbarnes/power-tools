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

    // IDK dude, maybe give AWK a go ... kinda fucked on MAC OS
    let mut sed = Command::new("sed");

    for (ln, line) in BufReader::new(yaml_file).lines().enumerate() {
        if let Some('#') = line
            .as_ref()
            .expect("could not match the line ref")
            .chars()
            .next()
        {
            let cmd = format!("{}s/../*/ {}", ln, yaml_path);
            sed.arg(cmd).spawn();

        } else {
        
            // -i may not be used with stdin??
            // I'm calling the fucking file
            let cmd = format!("$'{}i\\\n# ' {}", ln, yaml_path);
            sed.arg(cmd).spawn();
        }
    }

    Ok(())
}
