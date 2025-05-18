use opener::open;
use prettytable::{Cell, Row, Table};
use std::fs::write;
use crate::pathing;

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

    let styled_html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Directory Listing</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
    <style>
        body {{
            padding: 20px;
        }}
        h1 {{
            margin-bottom: 20px;
            color: #333;
        }}
        .table-responsive {{
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Directory Contents</h1>
        <div class="table-responsive">
            {}
        </div>
    </div>
</body>
</html>"#,
        String::from_utf8(buffer).unwrap()
    );

    write("table.html", styled_html).expect("Failed to write file");
    open("table.html").expect("Failed to open in browser");
}
