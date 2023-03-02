mod grid;
mod solver;

pub use grid::Grid;
pub use solver::SudokuSolver;

#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use crate::solver::SudokuSolver;

    #[test]
    fn solve_sudoku() {
        let grid = Grid::from([
            0, 0, 9, 0, 0, 1, 0, 7, 0, // first row
            0, 0, 0, 4, 0, 0, 0, 6, 0, // second row
            5, 0, 0, 0, 0, 2, 9, 0, 4, // third row
            0, 3, 0, 0, 0, 5, 0, 0, 0, // fourth row
            0, 0, 6, 2, 0, 0, 1, 0, 8, // fifth row
            0, 0, 0, 0, 0, 0, 0, 4, 0, // sixth row
            0, 0, 0, 0, 0, 0, 7, 0, 0, // seventh row
            6, 0, 0, 0, 1, 0, 0, 0, 0, // eighth row
            0, 0, 1, 8, 0, 0, 2, 0, 9, // nineth row
        ]);

        let mut sudoku_solver = SudokuSolver::builder().grid(grid).build();
        let solved_sudoku = sudoku_solver.solve();
        assert_eq!(
            solved_sudoku,
            Ok(Grid::from([
                4, 6, 9, 3, 5, 1, 8, 7, 2, // first row
                2, 8, 3, 4, 7, 9, 5, 6, 1, // second row
                5, 1, 7, 6, 8, 2, 9, 3, 4, // third row
                8, 3, 4, 1, 9, 5, 6, 2, 7, // fourth row
                7, 5, 6, 2, 3, 4, 1, 9, 8, // fifth row
                1, 9, 2, 7, 6, 8, 3, 4, 5, // sixth row
                9, 4, 8, 5, 2, 3, 7, 1, 6, // seventh row
                6, 2, 5, 9, 1, 7, 4, 8, 3, // eighth row
                3, 7, 1, 8, 4, 6, 2, 5, 9, // nineth row
            ]))
        );
    }
}
