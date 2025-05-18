//use std::io::prelude::*;
mod pathing;
use pathing::is_path_valid;
use std::fs;
use std::path::Path;
mod html;

fn main() {
    let path = Path::new("./");

    if is_path_valid(path) {
        let entries = fs::read_dir(path).unwrap();
        let mut search_path = pathing::SearchPath::new();

        search_path.populate(entries);
        search_path.print_files_folders();

        html::create_table(&search_path);
    }

}
