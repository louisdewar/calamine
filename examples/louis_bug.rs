use std::env;
use std::path::PathBuf;

use calamine::{open_workbook_auto, Reader};

fn main() {
    let excel_file = env::args()
        .nth(1)
        .expect("Please provide an excel file to convert");

    let sheet_name = env::args()
        .nth(2)
        .expect("Expecting a sheet name as second argument");
    let excel_path = PathBuf::from(excel_file);

    let mut workbook = open_workbook_auto(&excel_path).unwrap();
    let range = workbook.worksheet_formula(&sheet_name).unwrap();
    let (end_row, end_column) = range.end().unwrap();
    for row in 0..=end_row {
        for column in 0..=end_column {
            print!(
                "{}",
                range.get_value((row, column)).map_or("(empty)", |v| v)
            );
            if column != end_column {
                print!(";")
            }
        }
        print!("\n")
    }
}
