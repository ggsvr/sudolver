use std::env;
use std::fs;
use sudolver::{
    cell::NumCell,
    grid::Grid,
    number::{Number, ParseNumberError},
    solver::Solver,
};
const USAGE: &str = "USAGE: sudolver <grid file>";
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(grid_file) = env::args().nth(1) else {
        eprintln!("{USAGE}");
        return ExitCode::SUCCESS;
    };
    let file = match fs::read_to_string(grid_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: could not open file: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut grid = Grid::new();
    let numbers = file.split_whitespace().map(|s| s.parse::<NumCell>());

    for (i, (n, c)) in numbers.zip(grid.cells_mut()).enumerate() {
        let cell = match n {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error parsing grid file in number {}: {e}", i + 1);
                return ExitCode::FAILURE;
            }
        };
        *c = cell;
    }
    let mut solver = Solver::new(grid);
    if solver.solve() {
        eprintln!("solved");
        println!("{}", solver.grid);
        ExitCode::SUCCESS
    } else {
        eprintln!("couldn't solve");
        println!("{}", solver.grid);
        ExitCode::FAILURE
    }
}
