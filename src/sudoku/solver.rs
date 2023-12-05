use super::base::{CELLS, SudokuBase, row_by_cell, column_by_cell, COLUMN_INDICES, field_by_cell, FIELD_INDICES, ROWS, ROW_INDICES};

fn set_impossible(base: &mut SudokuBase, cell: usize, val: usize) {
    if base.cells.get(cell as u8).has_val() {
        return;
    }
    base.cells.get_mut(cell as u8).set_impossible(val as u8);
    {
        let cell_field = field_by_cell(cell);
        let field = &mut base.field_meta[cell_field];
        if !field.has_specified(val) {
            field.set_possibility(val, field.get_possibility(val) as usize - 1);
        }
    }
    {
        let cell_row = row_by_cell(cell);
        let row = &mut base.row_meta[cell_row];
        if !row.has_specified(val) {
            row.set_possibility(val, row.get_possibility(val) as usize - 1);
        }
    }
    {
        let cell_column = column_by_cell(cell);
        let column = &mut base.column_meta[cell_column];
        if !column.has_specified(val) {
            column.set_possibility(val, column.get_possibility(val) as usize - 1);
        }
    }
}

fn set_cell_val(base: &mut SudokuBase, cell: usize, val: usize) {

}

pub fn solve(board: &mut [u8; CELLS]) {
    let mut base = SudokuBase::new(board);
    let mut changed = true;

    while changed {
        changed = false;
        for cell_idx in 0..CELLS {
            let cell = base.cells.get(cell_idx as u8);
            if cell.has_val() {
                let val = cell.get_val().unwrap();
                println!("has val: {}", val);
                {
                    let row_idx = row_by_cell(cell_idx);
                    let row = &mut base.row_meta[row_idx];
                    if !row.has_specified(val as usize) {
                        println!("specified!");
                        changed = true;
                        row.set_possibility(val as usize, 0);
                        for cell in 0..9 {
                            let cell = (cell_idx - cell_idx % 9) as usize + cell;
                            set_impossible(&mut base, cell, val as usize);
                        }
                    }
                }
                {
                    let column_idx = column_by_cell(cell_idx);
                    let column = &mut base.column_meta[column_idx];
                    if !column.has_specified(val as usize) {
                        changed = true;
                        column.set_possibility(val as usize, 0);
                        for cell in 0..9 {
                            let cell = COLUMN_INDICES[column_idx as usize][cell];
                            set_impossible(&mut base, cell, val as usize);
                        }
                    }
                }
                {
                    let field_idx = field_by_cell(cell_idx);
                    let field = &mut base.field_meta[field_idx];
                    if !field.has_specified(val as usize) {
                        changed = true;
                        field.set_possibility(val as usize, 0);
                        for cell in 0..9 {
                            let cell = FIELD_INDICES[field_idx as usize][cell];
                            set_impossible(&mut base, cell, val as usize);
                        }
                    }
                }
            }
        }
        /*for row_idx in 0..ROWS {
            let row = &base.row_meta[row_idx];
            for i in 0..9 {
                if row.get_possibility(i) == 1 {
                    changed = true;
                    for cell in 0..9 {
                        let cell = ROW_INDICES[row_idx as usize][cell];
                        
                    }
                }
            }
        }*/
    }
    for cell_idx in 0..CELLS {
        board[cell_idx] = base.cells.get(cell_idx as u8).get_val().unwrap_or(0);
    }
}
