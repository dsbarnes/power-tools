use std::process::Command;
use std::str;
use clap::App;
use std::fs::File;
use std::io::{BufReader, BufRead};
/*
 * The idea here was to call a sed command based on
 * the first character of the line.
 *
 * Problem is ...something? It could be the MacOS env, it could
 * be I'm doing it wrong in rust.
 * In any case, AWK is more interesting - Thus, I move on
 */

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

    for (ln, line) in BufReader::new(yaml_file).lines().enumerate() {
        if let Some('#') = line.chars().next()
        {
            // sub any two continious characters with '*'
            let cmd = format!("s/../*/ {}", 
                shellexpand::tilde("~/Documents/power-tools/comments.yaml"));

            // This just does not work ...
            // It works when called bare from the command line, but not from here
            sed.arg(cmd).status()
                .expect("Could not do the sed stuff");
        }
    }
    Ok(())
}
