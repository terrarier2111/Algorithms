use crate::sudoku::base::{ROW_INDICES, COLUMN_INDICES, FIELD_INDICES, field_by_cell};

mod base;
pub mod solver;
pub mod verifier;

pub fn print_constants() {
    println!("rows:");
    for row in ROW_INDICES {
        println!("{:?}", row);
    }
    println!("columns:");
    for column in COLUMN_INDICES {
        println!("{:?}", column);
    }
    println!("fields:");
    for (idx, field) in FIELD_INDICES.into_iter().enumerate() {
        println!("field {idx}:");
        for y in 0..3 {
            println!("{} {} {}", field[y * 3 + 0], field[y * 3 + 1], field[y * 3 + 2]);
        }
        for i in 0..9 {
            assert_eq!(idx, field_by_cell(field[i]));
        }
    }
}