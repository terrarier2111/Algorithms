use std::mem::{size_of, MaybeUninit, transmute};

pub const FIELDS: usize = 9;
pub const ROWS: usize = 9;
pub const COLUMNS: usize = 9;
pub const CELLS: usize = ROWS * COLUMNS;

pub const FIELD_INDICES: [[usize; 9]; 9] = {
    const fn construct_field_idx(start: usize) -> [usize; 9] {
        let major_off = start / 9;
        let minor_off = start % 9;
        let start = major_off * 27 + minor_off;
        [
            start + 0 + 0 * 9, start + 1 + 0 * 9, start + 2 + 0 * 9,
            start + 0 + 1 * 9, start + 1 + 1 * 9, start + 2 + 1 * 9,
            start + 0 + 2 * 9, start + 1 + 2 * 9, start + 2 + 2 * 9,
        ]
    }

    let mut ret = [[0; 9]; 9];
    let mut i = 0;
    while i < 9 {
        ret[i] = construct_field_idx(i * 3);
        i += 1;
    }
    ret
};

pub const ROW_INDICES: [[usize; 9]; 9] = {
    let mut ret = [[0; 9]; 9];
    let mut i = 0;
    while i < 9 {
        ret[i] = {
            let mut row = [0; 9];
            let mut k = 0;
            while k < 9 {
                row[k] = i * 9 + k;
                k += 1;
            }
            row
        };
        i += 1;
    }
    ret
};

pub const COLUMN_INDICES: [[usize; 9]; 9] = {
    let mut ret = [[0; 9]; 9];
    let mut i = 0;
    while i < 9 {
        ret[i] = {
            let mut column = [0; 9];
            let mut k = 0;
            while k < 9 {
                column[k] = i + k * 9;
                k += 1;
            }
            column
        };
        i += 1;
    }
    ret
};

#[inline]
pub const fn column_by_cell(cell: usize) -> usize {
    cell % 9
}

#[inline]
pub const fn row_by_cell(cell: usize) -> usize {
    cell / 9
}

#[inline]
pub const fn field_by_cell(cell: usize) -> usize {
    let field_vert = cell / 27;
    let field_horiz = (cell % 9) / 3;
    field_vert * 3 + field_horiz
}

pub struct SudokuBase {
    pub cells: CellArray,
    pub field_meta: [MetaArray; 9],
    pub column_meta: [MetaArray; 9],
    pub row_meta: [MetaArray; 9],
}

impl SudokuBase {

    // FIXME: the count in the row/column/field metadata is incorrect!
    pub fn new(board: &[u8; CELLS]) -> Self {
        let mut cells = CellArray::new(board);
        let mut column_meta = [MetaArray([0; META_ARRAY_LEN]); 9];
        // column meta
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                let cell = cells.get(COLUMN_INDICES[column][row] as u8);
                if cell.has_val() {
                    continue;
                }
                for val in cell.possible_vals() {
                    column_meta[column].set_possibility(val as usize, column_meta[column].get_possibility(val as usize) as usize + 1);
                }
            }
        }

        let mut row_meta = [MetaArray([0; META_ARRAY_LEN]); 9];
        // row meta
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                let cell = cells.get(ROW_INDICES[row][column] as u8);
                if cell.has_val() {
                    continue;
                }
                for val in cell.possible_vals() {
                    row_meta[row].set_possibility(val as usize, row_meta[row].get_possibility(val as usize) as usize + 1);
                }
            }
        }

        let mut field_meta = [MetaArray([0; META_ARRAY_LEN]); 9];
        // field meta
        for field in 0..FIELDS {
            for idx in 0..9 {
                let cell = cells.get(FIELD_INDICES[field][idx] as u8);
                if cell.has_val() {
                    continue;
                }
                for val in cell.possible_vals() {
                    field_meta[field].set_possibility(val as usize, field_meta[field].get_possibility(val as usize) as usize + 1);
                }
            }
        }

        Self {
            cells,
            field_meta,
            column_meta,
            row_meta,
        }
    }

}

const META_ARRAY_LEN: usize = 9_usize.div_ceil(2 * size_of::<u64>() / size_of::<u8>());

#[derive(Clone, Copy)]
pub struct MetaArray([u64; META_ARRAY_LEN]);

impl MetaArray {

    const BASE: u64 = (1 << 4) - 1;

    #[inline]
    pub fn has_specified(&self, num: usize) -> bool {
        self.get_possibility(num) == 0
    }

    #[inline]
    pub fn get_possibility(&self, num: usize) -> u8 {
        let top_idx = num / (2 * size_of::<u64>() / size_of::<u8>());
        let sub_idx = num % (2 * size_of::<u64>() / size_of::<u8>());
        ((self.0[top_idx] & (Self::BASE << (4 * sub_idx))) >> (4 * sub_idx)) as u8
    }

    #[inline]
    pub const fn set_possibility(&mut self, num: usize, val: usize) {
        let top_idx = num / (2 * size_of::<u64>() / size_of::<u8>());
        let sub_idx = num % (2 * size_of::<u64>() / size_of::<u8>());
        self.0[top_idx] &= !(Self::BASE << (4 * sub_idx));
        self.0[top_idx] |= (val as u64) << (4 * sub_idx as u64);
    }

}

pub struct CellArray([Cell; CELLS]);

impl CellArray {

    #[inline]
    const fn new(board: &[u8; CELLS]) -> Self {
        let mut raw = [MaybeUninit::uninit(); CELLS];
        let mut idx = 0;
        while idx < board.len() {
            if board[idx] != 0 {
                raw[idx].write(Cell::new_val(board[idx]));
            } else {
                let mut maybe = Cell::ANY;
                // row elems
                let row = row_by_cell(idx);
                let mut i = 0;
                while i < 9 {
                    let val = board[ROW_INDICES[row][i]];
                    if val != 0 {
                        maybe &= !(1 << val);
                    }
                    i += 1;
                }
                // column elems
                let column = column_by_cell(idx);
                let mut i = 0;
                while i < 9 {
                    let val = board[COLUMN_INDICES[column][i]];
                    if val != 0 {
                        maybe &= !(1 << val);
                    }
                    i += 1;
                }
                // field elems
                let field = field_by_cell(idx);
                let mut i = 0;
                while i < 9 {
                    let val = board[FIELD_INDICES[field][i]];
                    if val != 0 {
                        maybe &= !(1 << val);
                    }
                    i += 1;
                }
                raw[idx].write(Cell::new_maybe(maybe));
            }
            idx += 1;
        }
            
        Self(unsafe { transmute(raw) })
    }

    #[inline]
    pub const fn get(&self, idx: u8) -> Cell {
        self.0[idx as usize]
    }

    #[inline]
    pub const fn get_mut(&mut self, idx: u8) -> &mut Cell {
        &mut self.0[idx as usize]
    }

}

#[derive(Clone, Copy)]
pub struct Cell(u16);

impl Cell {

    const ANY: u16 = ((1 << 10) - 1) & !1;

    #[inline]
    const fn new_val(num: u8) -> Self {
        Self(1 << num)
    }

    #[inline]
    const fn new_maybe(maybe: u16) -> Self {
        Self(maybe)
    }

    #[inline]
    pub const fn possible_vals_set(self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn possible_vals(self) -> PossibleValsIter {
        PossibleValsIter(self.0)
    }
    
    #[inline]
    pub const fn may_be(self, val: u8) -> bool {
        self.0 & (1 << val) != 0
    }

    #[inline]
    pub const fn set_impossible(&mut self, val: u8) {
        self.0 &= !(1 << val as u16);
    }

    #[inline]
    pub const fn set_val(&mut self, val: u8) {
        self.0 = 1 << val as u16;
    }

    #[inline]
    pub const fn get_val(self) -> Option<u8> {
        if self.0.count_ones() > 1 {
            return None;
        }
        Some(self.0.trailing_zeros() as u8)
    }

    #[inline]
    pub const fn has_val(self) -> bool {
        self.0.count_ones() == 1
    }

}

pub struct PossibleValsIter(u16);

impl Iterator for PossibleValsIter {

    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let idx = self.0.trailing_zeros();
        self.0 &= !(1 << idx);
        Some(idx as u8)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.count_ones() as usize, Some(self.0.count_ones() as usize))
    }

}

impl ExactSizeIterator for PossibleValsIter {}
