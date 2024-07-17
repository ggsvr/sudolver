use crate::cell::*;
use crate::grid::*;
use crate::number::Number;

#[derive(Debug, Clone)]
pub struct Solver {
    pub grid: Grid,
    backups: Vec<Grid>,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            backups: Vec::new(),
        }
    }
    /// tries to solve its `Grid`, returning `true` on success
    /// and `false` on failure
    pub fn solve(&mut self) -> bool {
        // walk through all collapsed positions and propagate constraints
        for (x, y) in Grid::positions() {
            let Some(num) = self.grid.get(x, y).unwrap().number() else {
                continue;
            };
            self.propagate(x, y, num);
        }

        // do this until it's solved
        loop {
            // if there's no uncertain cells left, we're done
            let (x, y, cell) = match self.least_entropic() {
                None => return true,
                Some(c) => c,
            };
            let e = cell.entropy();
            match e {
                // if entropy is 1, just collapse and propagate
                1 => {
                    let num = cell.collapse();
                    *self.grid.get_mut(x, y).unwrap() = NumCell::Collapsed(num);
                    self.propagate(x, y, num);
                }
                // If entropy is 0, it means we guessed something wrong.
                // Check for backups. If there isn't any, the initial grid
                // was unsolvable.
                0 => match self.backups.pop() {
                    None => return false,
                    Some(b) => self.grid = b,
                },
                // if entropy is bigger than one, we make a backup and collapse
                // the cell to a random possible number, excluding this number
                // from the backup cell.
                _ => {
                    let num = cell.some_element().unwrap();
                    let mut bak = self.grid.clone();
                    bak.get_mut(x, y)
                        .unwrap()
                        .if_uncertain_mut(|c| c.exclude(num));
                    self.backups.push(bak);

                    *self.grid.get_mut(x, y).unwrap() = NumCell::Collapsed(num);
                    self.propagate(x, y, num);
                }
            }
        }
    }
    /// Removes `num` from the row, column and group containing this position.
    fn propagate(&mut self, x: u8, y: u8, num: Number) {
        self.grid.row_mut(y).unwrap().exclude(num);
        self.grid.col_mut(x).unwrap().exclude(num);
        self.grid.group_containing_mut(x, y).unwrap().exclude(num);
    }
    /// gets the least entropic uncertain cell
    /// None is return if there are no uncertain cells left
    fn least_entropic(&mut self) -> Option<(u8, u8, &mut UncertainCell)> {
        self.grid
            .cells_xy_mut()
            .filter_map(|(x, y, cell)| cell.uncertain_mut().map(|c| (x, y, c)))
            .reduce(|acc, cell| {
                if cell.2.entropy() < acc.2.entropy() {
                    cell
                } else {
                    acc
                }
            })
    }
}
