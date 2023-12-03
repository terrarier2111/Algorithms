use super::base::{CELLS, SudokuBase, row_by_cell, column_by_cell, COLUMNS, COLUMN_INDICES, field_by_cell, FIELD_INDICES};

pub fn solve(board: &mut [u8; CELLS]) {
    let mut base = SudokuBase::new(board);
    loop {
        for cell_idx in 0..CELLS {
            let cell = base.cells.get_mut(cell_idx as u8);
            let val = cell.get_val().unwrap();
            if !cell.has_val() {
                let row_idx = row_by_cell(cell_idx);
                let row = &base.row_meta[row_idx];
                row.set_possibility(val as usize, 0);
                for cell in 0..9 {
                    base.cells.get_mut((cell_idx - cell_idx % 9) as u8).set_impossible(val as u8);
                }

                let column_idx = column_by_cell(cell_idx);
                let column = &base.column_meta[column_idx];
                column.set_possibility(val as usize, 0);
                for cell in 0..9 {
                    base.cells.get_mut(COLUMN_INDICES[column_idx as usize][cell] as u8).set_impossible(val as u8);
                }

                let field_idx = field_by_cell(cell_idx);
                let field = &base.field_meta[column_idx];
                field.set_possibility(val as usize, 0);
                for cell in 0..9 {
                    base.cells.get_mut(FIELD_INDICES[field_idx as usize][cell] as u8).set_impossible(val as u8);
                }
            }
        }
    }
}
