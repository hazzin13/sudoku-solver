use crate::grid::{Grid, GRID_SIZE, SQUARE_SIZE};

#[derive(Debug)]
pub struct SudokuSolver {
    grid: Grid,
}

impl SudokuSolver {
    pub fn builder() -> SolverBuilder {
        SolverBuilder {
            grid: Default::default(),
        }
    }

    fn empty_cells(&self) -> std::vec::IntoIter<u8> {
        let empty_cells = self
            .grid
            .iter()
            .enumerate()
            .filter(|&(_, cell)| cell == 0)
            .map(|(index, _)| index as u8)
            .collect::<Vec<_>>();
        empty_cells.into_iter()
    }

    fn check_row(&self, row: usize, number: u8) -> bool {
        let mut invalid = false;
        for column in 0..GRID_SIZE {
            if self.grid.get_cell(row, column) == number {
                invalid = true;
                break;
            }
        }
        invalid
    }

    fn check_column(&self, column: usize, number: u8) -> bool {
        let mut invalid = false;
        for row in 0..GRID_SIZE {
            if self.grid.get_cell(row, column) == number {
                invalid = true;
                break;
            }
        }
        invalid
    }

    fn check_square(&self, row: usize, column: usize, number: u8) -> bool {
        let mut invalid = false;
        for x in row / SQUARE_SIZE * SQUARE_SIZE..row / SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE {
            if invalid {
                break;
            }

            for y in
                column / SQUARE_SIZE * SQUARE_SIZE..column / SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE
            {
                if self.grid.get_cell(x, y) == number {
                    invalid = true;
                    break;
                }
            }
        }
        invalid
    }

    fn solve_cell(&mut self, mut empty_cells: std::vec::IntoIter<u8>) -> bool {
        let index = match empty_cells.next() {
            None => return true,
            Some(index) => index as usize,
        };

        let current_row = index / GRID_SIZE;
        let current_column = index % GRID_SIZE;

        for num in 1..=GRID_SIZE as u8 {
            if self.check_row(current_row, num) {
                continue;
            }

            if self.check_column(current_column, num) {
                continue;
            }

            if self.check_square(current_row, current_column, num) {
                continue;
            }

            self.grid.set_cell(index, num);

            match self.solve_cell(empty_cells.clone()) {
                false => self.grid.set_cell(index, 0),
                true => return true,
            };
        }

        false
    }

    pub fn solve(&mut self) -> Result<Grid, ()> {
        match self.solve_cell(self.empty_cells()) {
            false => Err(()),
            true => Ok(self.grid),
        }
    }
}

pub struct SolverBuilder {
    grid: Grid,
}

impl SolverBuilder {
    pub fn grid(mut self, grid: Grid) -> Self {
        self.grid = grid;
        self
    }

    pub fn build(self) -> SudokuSolver {
        SudokuSolver { grid: self.grid }
    }
}
