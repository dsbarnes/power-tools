use clap::App;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> Result<(), &'static str>{
    let args = App::new("swap")
        .args_from_usage(
            "<PATH>     'path to config.yaml'"
            )
        .get_matches();
    let path = args.value_of("PATH").expect("
        Could not get the path");
    let yaml_file = File::open(path)
        .expect("Could not open the yaml_path");

    for (ln, line) in BufReader::new(yaml_file).lines().enumerate() {

    }

    Ok(())
}
