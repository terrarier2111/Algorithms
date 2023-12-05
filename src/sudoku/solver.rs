use super::base::{CELLS, SudokuBase, row_by_cell, column_by_cell, COLUMN_INDICES, field_by_cell, FIELD_INDICES, ROWS, ROW_INDICES, COLUMNS, FIELDS};

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
    let vals = base.cells.get_mut(cell as u8).possible_vals();
    base.cells.get_mut(cell as u8).set_val(val as u8);
    for val in vals {
        let val = val as usize;
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
                // println!("has val: {}", val);
                {
                    let row_idx = row_by_cell(cell_idx);
                    let row = &mut base.row_meta[row_idx];
                    if !row.has_specified(val as usize) {
                        println!("specified 1!");
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
                        println!("specified 2!");
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
                        println!("specified 3!");
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
        for row_idx in 0..ROWS {
            for i in 1..10 {
                if base.row_meta[row_idx].get_possibility(i) == 1 {
                    println!("update row {}|{}", row_idx, i);
                    changed = true;
                    for cell in 0..9 {
                        let cell = ROW_INDICES[row_idx as usize][cell];
                        if base.cells.get(cell as u8).may_be(i as u8) {
                            set_cell_val(&mut base, cell, i);
                        }
                    }
                }
            }
        }
        for column_idx in 0..COLUMNS {
            for i in 1..10 {
                if base.column_meta[column_idx].get_possibility(i) == 1 {
                    println!("update column {}|{}", column_idx, i);
                    changed = true;
                    for cell in 0..9 {
                        let cell = COLUMN_INDICES[column_idx as usize][cell];
                        if base.cells.get(cell as u8).may_be(i as u8) {
                            set_cell_val(&mut base, cell, i);
                        }
                    }
                }
            }
        }
        for field_idx in 0..FIELDS {
            for i in 1..10 {
                if base.field_meta[field_idx].get_possibility(i) == 1 {
                    println!("update field {}|{}", field_idx, i);
                    changed = true;
                    for cell in 0..9 {
                        let cell = FIELD_INDICES[field_idx as usize][cell];
                        if base.cells.get(cell as u8).may_be(i as u8) {
                            set_cell_val(&mut base, cell, i);
                        }
                    }
                }
            }
        }
    }
    for cell_idx in 0..CELLS {
        board[cell_idx] = base.cells.get(cell_idx as u8).get_val().unwrap_or(0);
    }
}
