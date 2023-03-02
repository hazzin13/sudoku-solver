use sudoku_solver::Grid;
use sudoku_solver::SudokuSolver;

fn main() {
    // let grid = Grid::from([
    //     0, 1, 0, 0, 6, 0, 0, 0, 0, // first row
    //     0, 0, 8, 0, 0, 0, 3, 0, 0, // second row
    //     5, 0, 0, 1, 0, 7, 0, 0, 9, // third row
    //     0, 0, 0, 4, 0, 0, 0, 0, 0, // fourth row
    //     1, 0, 0, 9, 0, 2, 0, 0, 7, // fifth row
    //     0, 5, 0, 0, 0, 0, 0, 1, 0, // sixth row
    //     0, 3, 0, 2, 0, 6, 9, 0, 0, // seventh row
    //     0, 0, 0, 0, 5, 0, 0, 0, 6, // eighth row
    //     2, 0, 0, 0, 4, 0, 0, 0, 0, // nineth row
    // ]);

    // let grid = Grid::from([
    //     5, 0, 1, 2, 0, 7, 0, 8, 0, // first row
    //     6, 4, 0, 1, 0, 3, 2, 0, 0, // second row
    //     0, 2, 7, 9, 5, 0, 6, 0, 3, // third row
    //     7, 0, 0, 4, 0, 0, 0, 9, 6, // fourth row
    //     0, 0, 6, 0, 0, 0, 8, 0, 2, // fifth row
    //     2, 0, 8, 0, 3, 9, 7, 0, 0, // sixth row
    //     0, 0, 0, 0, 4, 0, 3, 0, 8, // seventh row
    //     3, 7, 0, 8, 0, 0, 0, 6, 1, // eighth row
    //     1, 0, 0, 0, 0, 0, 4, 0, 0, // nineth row
    // ]);

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
    match sudoku_solver.solve() {
        Err(_) => println!("Failed"),
        Ok(solved_grid) => {
            println!("{}", &solved_grid);
        }
    };
}
