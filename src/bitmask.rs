pub struct Bitmask {
    pub rows: [u16; 9],
    pub cols: [u16; 9],
    pub sub_grids: [u16; 9],
}

// Initilize Bitmask to default = 0
impl Default for Bitmask {
    fn default() -> Self {
        Self {
            rows: [0; 9],
            cols: [0; 9],
            sub_grids: [0; 9],
        }
    }
}

impl Bitmask {
    pub fn new(grid: &[[u8; 9]; 9]) -> Self {
        // Initialise Bitmask values to 0
        let mut bitmask = Bitmask::default(); 

        for row in 0..9 {
            for col in 0..9 {
                let value = grid[row][col];

                if value != 0 {
                    bitmask.add_value(row, col, value);
                }
            }
        }

        bitmask
    }

    fn add_value(&mut self, row_index: usize, col_index: usize, value: u8) {
        let bit: u16 = 1 << (value - 1);
        let sub_grid_index: usize = (row_index / 3) * 3 + (col_index / 3);

        self.rows[row_index] |= bit;
        self.cols[col_index] |= bit;
        self.sub_grids[sub_grid_index] |= bit;
    }
}
