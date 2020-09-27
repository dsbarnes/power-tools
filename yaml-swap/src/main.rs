use std::fs::File;
use std::fs;

/*
 * This is the basic program I want to write by calling
 * sed and awk from inside rust (std::process::Command)
 *
 * Simply flip what is and is not commented within a yaml file.
 *
 */

fn comment_line(text: String) -> String {
    let mut comment = "# ".to_string();
    comment.push_str(text.as_str());
    comment
}

fn uncomment_line(text: String) -> String {
    let mut new_text = String::with_capacity(text.len());
    for (idx, letter) in text.chars().enumerate() {
        if idx > 1 {
            new_text.push(letter);
        }
    }
    new_text
}

fn main() -> Result<(), &'static str>{
    // This is much better done with buffers to avoid reading
    // the whole file into mem at once (like that matters in this case)
    if cfg!(target_os = "windows") {
        return 
        Err("This program does not work for windows, and I don't plan to port it") }

    let path = shellexpand::tilde("~/Documents/Dev/rustDev/power-tools/comments.yaml");
    let yaml = fs::read_to_string(path.as_ref())
        .expect("Could not read the file into a string");
    let mut swaped_file = vec![];
    let lines = yaml.split('\n');
    for line in lines {
        if let Some('#') = line.chars().next() {
            let new_line = uncomment_line(line.to_string());
            swaped_file.push(new_line);
        } else {
            let new_line = comment_line(line.to_string());
            swaped_file.push(new_line);
        }
    }

    let swapped_file = swaped_file.join("\n");
    fs::write(path.as_ref(), swapped_file)
        .expect("Could not write to the file");

    Ok(())
}
