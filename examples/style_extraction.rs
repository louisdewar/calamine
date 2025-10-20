use calamine::{open_workbook, Reader, ReaderRef, Xlsx};
use std::env;
use std::path::PathBuf;

fn main() {
    let excel_file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example style_extraction <path_to_xlsx_file> [sheet_name]");
        std::process::exit(1);
    });

    let sheet_name = env::args().nth(2);
    let excel_path = PathBuf::from(excel_file);

    let mut workbook: Xlsx<_> = open_workbook(&excel_path).expect("Failed to open workbook");

    // Print available styles
    println!("=== Workbook Styles ===");
    println!("Total styles defined: {}", workbook.styles.len());
    for (idx, style) in workbook.styles.iter().enumerate() {
        println!("Style {}: {:?}", idx, style);
    }
    println!();

    // Print number formats
    println!("=== Number Formats ===");
    for (id, format_str) in &workbook.number_formats {
        println!("Format {}: {}", id, format_str);
    }
    println!();

    // Get sheet name (first sheet if not specified)
    let sheet = if let Some(name) = sheet_name {
        name
    } else {
        workbook
            .sheet_names()
            .first()
            .expect("No sheets found")
            .clone()
    };

    println!("=== Reading cells from sheet: {} ===", sheet);

    // Read cells using the reference API to get style indices
    let range = workbook
        .worksheet_range_ref(&sheet)
        .expect("Failed to read worksheet");

    // Show first 10 non-empty cells with their styles
    let mut count = 0;
    for (row, col, value) in range.cells() {
        if count >= 10 {
            break;
        }

        // For demonstration, print cells at row 0 with their styles
        if row == 0 {
            println!("Cell ({}, {}): {:?}", row, col, value);
            count += 1;
        }
    }

    if count == 0 {
        println!("No cells found in row 0.");
    }

    println!("\nNote: Cell style indices are stored internally but Range API doesn't expose them directly.");
    println!("To access style information, you would need to use the worksheet_cells_reader API.");
}
