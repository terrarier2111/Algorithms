use super::base::{CELLS, ROWS, COLUMNS, ROW_INDICES, COLUMN_INDICES, FIELDS, FIELD_INDICES};

const EXPECTED: usize = ((1 << 10) - 1) & !1;

pub fn verify(grid: &[u8; CELLS]) -> bool {
    for row in 0..ROWS {
        let mut vals = 0;
        for column in 0..COLUMNS {
            vals |= 1 << grid[ROW_INDICES[row][column]];
        }
        if vals != EXPECTED {
            return false;
        }
    }
    for column in 0..COLUMNS {
        let mut vals = 0;
        for row in 0..ROWS {
            vals |= 1 << grid[COLUMN_INDICES[column][row]];
        }
        if vals != EXPECTED {
            return false;
        }
    }
    for field in 0..FIELDS {
        let mut vals = 0;
        for entry in 0..9 {
            vals |= 1 << grid[FIELD_INDICES[field][entry]];
        }
        if vals != EXPECTED {
            return false;
        }
    }
    true
}


mod tests {
    use super::verify;


    const CORRECT: &[u8; 81] = &[
        7, 6, 1, 8, 9, 2, 3, 5, 4,
        4, 9, 8, 5, 3, 6, 7, 2, 1,
        2, 5, 3, 4, 1, 7, 9, 8, 6,
        5, 3, 4, 1, 6, 8, 2, 9, 7,
        8, 1, 7, 2, 5, 9, 4, 6, 3,
        6, 2, 9, 3, 7, 4, 5, 1, 8,
        1, 8, 5, 9, 4, 3, 6, 7, 2,
        9, 4, 6, 7, 2, 1, 8, 3, 5,
        3, 7, 2, 6, 8, 5, 1, 4, 9,
    ];

    const INCORRECT: &[u8; 81] = &[
        7, 6, 1, 8, 9, 2, 3, 5, 4,
        4, 9, 8, 5, 3, 6, 7, 2, 1,
        2, 5, 3, 4, 1, 7, 9, 8, 6,
        5, 3, 4, 5, 6, 8, 2, 9, 7,
        8, 1, 7, 2, 5, 9, 4, 6, 3,
        6, 2, 9, 3, 7, 4, 5, 1, 8,
        1, 8, 5, 9, 4, 3, 6, 7, 2,
        9, 4, 6, 7, 2, 1, 8, 3, 5,
        3, 7, 2, 6, 8, 5, 1, 4, 9,
    ];

    #[test]
    fn test_verify() {
        assert!(verify(CORRECT));
        assert!(!verify(INCORRECT));
    }

}
