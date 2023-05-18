// main.rs
// use crate::{crate_info::CrateInfo};
// use nodium_pdk::{TableView, TableRow, TableCell}

// pub fn crate_view_table(crates: Vec<CrateInfo>) -> TableView {
//     let mut table = TableView::new()
//         .header(TableRow::new(vec![
//             TableCell::new("ID"),
//             TableCell::new("Name"),
//             TableCell::new("Description"),
//             TableCell::new("Last Updated"),
//             TableCell::new("Downloads"),
//         ]));

//     for c in crates {
//         let row = TableRow::new(vec![
//             TableCell::new(c.id),
//             TableCell::new(c.name),
//             TableCell::new(c.description),
//             TableCell::new(c.updated_at),
//             TableCell::new(c.downloads.to_string()),
//         ]);

//         table.add_row(row);
//     }

//     table
// }
