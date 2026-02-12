pub struct Universe {
    pub cells: Vec<bool>,
    pub rows: u32,
    pub cols: u32,
}

impl Universe {
    pub fn new(rows: u32, cols: u32, dna: &[u8]) -> Self {
        let mut cells = vec![false; (rows * cols) as usize];
        
        // Seed the cells based on DNA sequence
        // G/C bases create "Alive" cells
        for (i, &base) in dna.iter().enumerate() {
            if i >= cells.len() { break; }
            if base == b'G' || base == b'C' {
                cells[i] = true;
            }
        }
        
        Self { cells, rows, cols }
    }

    pub fn toggle(&mut self, row: u32, col: u32) {
        let idx = (row * self.cols + col) as usize;
        self.cells[idx] = !self.cells[idx];
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let live_neighbors = self.live_neighbor_count(row, col);
                let idx = (row * self.cols + col) as usize;

                let next_state = match (self.cells[idx], live_neighbors) {
                    (true, x) if x < 2 => false,   // Underpopulation
                    (true, 2) | (true, 3) => true, // Survival
                    (true, x) if x > 3 => false,   // Overpopulation
                    (false, 3) => true,            // Birth
                    (otherwise, _) => otherwise,   // Stay same
                };

                next[idx] = next_state;
            }
        }
        self.cells = next;
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.rows - 1, 0, 1].iter().cloned() {
            for delta_col in [self.cols - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 { continue; }

                let neighbor_row = (row + delta_row) % self.rows;
                let neighbor_col = (col + delta_col) % self.cols;
                let idx = (neighbor_row * self.cols + neighbor_col) as usize;
                if self.cells[idx] { count += 1; }
            }
        }
        count
    }
}
