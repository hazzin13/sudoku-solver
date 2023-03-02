pub const SQUARE_SIZE: usize = 3;
pub const GRID_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Grid {
    cells: [u8; GRID_SIZE * GRID_SIZE],
}

impl Grid {
    pub fn iter(&self) -> GridIterator {
        GridIterator {
            index: 0,
            grid: *self,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.cells[x * GRID_SIZE + y]
    }

    pub fn set_cell(&mut self, index: usize, value: u8) {
        self.cells[index] = value;
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: [0; GRID_SIZE * GRID_SIZE],
        }
    }
}

impl From<[u8; GRID_SIZE * GRID_SIZE]> for Grid {
    fn from(cells: [u8; GRID_SIZE * GRID_SIZE]) -> Self {
        Self { cells }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sudoku = String::new();

        for (index, number) in self.cells.into_iter().enumerate() {
            if index % GRID_SIZE == 0 {
                sudoku.push('\n');
            } else {
                sudoku.push(' ');
            }

            if number != 0 {
                sudoku.push(char::from_digit(number as u32, 10).unwrap());
            } else {
                sudoku.push('-');
            }
        }

        write!(f, "{sudoku}")
    }
}

pub struct GridIterator {
    index: usize,
    grid: Grid,
}

impl Iterator for GridIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.cells.len() {
            None
        } else {
            let item = self.grid.cells[self.index];
            self.index += 1;
            Some(item)
        }
    }
}
