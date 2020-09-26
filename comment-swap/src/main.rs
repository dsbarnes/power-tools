use std::process::Command;
use std::str;
use clap::App;

// Doesn't look like we need this? that's new
//shellexpand::tilde();

fn main() -> Result<(), &'static str> {
    if cfg!(target_os = "windows") {
        return 
        Err("This program does not work for windows, and I don't plan to port it") }

    let args = App::new("comment-swap")
        .args_from_usage(
            "<FILE>     'The file to swap'"
        )
        .get_matches();

    let yaml_file = args.value_of("FILE")
        .expect("Could not read the file");

    println!("The config file is: {}", yaml_file);

    Ok(())
}
