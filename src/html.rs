use opener::open;
use prettytable::{Cell, Row, Table};
use std::fs::{self, write};
use crate::pathing;

// Creates html table of files & folders in search path
pub fn create_table(search_path: &pathing::SearchPath) {
    let mut table = Table::new();

    // Header
    table.add_row(Row::new(vec![Cell::new("Type"), Cell::new("Name")]));

    for name in search_path.files.iter() {
        table.add_row(Row::new(vec![
            Cell::new("File"),
            Cell::new(&name.to_string_lossy()),
        ]));
    }

    for name in search_path.folders.iter() {
        table.add_row(Row::new(vec![
            Cell::new("Folder"),
            Cell::new(&name.to_string_lossy()),
        ]));
    }

    let mut buffer = Vec::new();
    table.print_html(&mut buffer).expect("Failed to write HTML");
    let html_table = String::from_utf8(buffer).unwrap();

    let template = fs::read_to_string("src/template/table.html").expect("Failed to load in template");
    let styled_html = template.replace("<!-- TABLE_PLACEHOLDER -->", &html_table); 


    write("test_table.html", styled_html).expect("Failed to write file");
    open("test_table.html").expect("Failed to open in browser");
}
