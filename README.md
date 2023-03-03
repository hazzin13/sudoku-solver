# Sudoku solver implemented in Rust

This sudoku solver is a simple implementation of a backtracking algorithm

## Example

```rust
use sudoku_solver::{Grid, SudokuSolver};

fn main() {
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
```
