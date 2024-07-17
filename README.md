# Sudolver

A simple sudoku solver written in Rust.

## INSTALLING
Just run `cargo install --git <repo url>`.


## USAGE
`sudolver` accepts a grid file as input, containing 81 numbers
separated by whitespace, so the file layout is flexible.

For a blank cell, use one of the characters: [`,` `.` `=` `-`].

### Example
example.grid
```
- - 7   - 8 -   - - -
4 - -   - - -   - 2 -
- - 6   - - 9   - 1 -

- - -   - - 6   - - 9
5 7 -   - - -   - - 8
- 9 -   - 5 3   - - -

- - -   5 - -   2 - -
2 - -   9 - -   - 7 -
- - 9   2 - 7   6 8 -
```

command: `sudolver example.grid`


## API
If using as a library, `sudolver` exposes the following items:

- `struct Solver`
- `struct Grid`
- `enum NumCell`
- `struct Number`

Construct a starting `Grid` with `NumCell::Collapsed` in the starting
positions, and solve it with `Solver::solve()`.
