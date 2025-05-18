use std::io::{stdin};
mod pathing;
use pathing::is_path_valid;
use std::fs;
use std::path::Path;
mod html;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();
    println!("User input: {}", input);
    let path = Path::new(input);

    if is_path_valid(path) {
        let entries = fs::read_dir(path).unwrap();
        let mut search_path = pathing::SearchPath::new();

        search_path.populate(entries);
        search_path.print_files_folders();

        html::create_table(&search_path);
    }
}
