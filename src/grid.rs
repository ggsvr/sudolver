use crate::cell::NumCell;
use crate::number::Number;

const GRID_SIDE: usize = 9;
const GRID_LEN: usize = GRID_SIDE * GRID_SIDE;
#[derive(Debug, Clone)]
pub struct Grid {
    cells: Box<[NumCell]>,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            cells: vec![NumCell::new(); GRID_LEN].into_boxed_slice(),
        }
    }
    pub fn get(&self, x: u8, y: u8) -> Option<&NumCell> {
        self.cells.get(y as usize * GRID_SIDE + x as usize)
    }
    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut NumCell> {
        self.cells.get_mut(y as usize * GRID_SIDE + x as usize)
    }
    pub fn row(&self, row_num: u8) -> Option<Row> {
        if row_num >= 9 {
            return None;
        }
        Some(Row {
            grid: self,
            y: row_num,
        })
    }
    pub fn row_mut(&mut self, row_num: u8) -> Option<RowMut> {
        if row_num >= 9 {
            return None;
        }
        Some(RowMut {
            grid: self,
            y: row_num,
        })
    }
    pub fn col(&self, col_num: u8) -> Option<Column> {
        if col_num >= 9 {
            return None;
        }
        Some(Column {
            grid: self,
            x: col_num,
        })
    }
    pub fn col_mut(&mut self, col_num: u8) -> Option<ColumnMut> {
        if col_num >= 9 {
            return None;
        }
        Some(ColumnMut {
            grid: self,
            x: col_num,
        })
    }
    pub fn group(&self, group_num: u8) -> Option<Group> {
        let (x, y) = nth_group_xy(group_num)?;
        Some(Group { grid: self, x, y })
    }
    pub fn group_mut(&mut self, group_num: u8) -> Option<GroupMut> {
        let (x, y) = nth_group_xy(group_num)?;
        Some(GroupMut { grid: self, x, y })
    }
    pub fn group_containing(&self, x: u8, y: u8) -> Option<Group> {
        let (x, y) = group_xy_containing_xy(x, y)?;
        Some(Group { grid: self, x, y })
    }
    pub fn group_containing_mut(&mut self, x: u8, y: u8) -> Option<GroupMut> {
        let (x, y) = group_xy_containing_xy(x, y)?;
        Some(GroupMut { grid: self, x, y })
    }
    pub fn cells(&self) -> std::slice::Iter<NumCell> {
        self.cells.iter()
    }
    pub fn cells_mut(&mut self) -> std::slice::IterMut<NumCell> {
        self.cells.iter_mut()
    }
    pub fn positions() -> impl Iterator<Item = (u8, u8)> {
        (0..GRID_LEN).map(|i| ((i % GRID_SIDE) as u8, (i / GRID_SIDE) as u8))
    }

    pub fn cells_xy(&self) -> impl Iterator<Item = (u8, u8, &NumCell)> {
        self.cells.iter().enumerate().map(|(i, cell)| {
            let x = i % GRID_SIDE;
            let y = i / GRID_SIDE;
            (x as u8, y as u8, cell)
        })
    }
    pub fn cells_xy_mut(&mut self) -> impl Iterator<Item = (u8, u8, &mut NumCell)> {
        self.cells.iter_mut().enumerate().map(|(i, cell)| {
            let x = i % GRID_SIDE;
            let y = i / GRID_SIDE;
            (x as u8, y as u8, cell)
        })
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, y, cell) in self.cells_xy() {
            let n = cell.number().map(|n| n.number());

            if x % 3 == 0 {
                write!(f, "\t")?;
            }
            match n {
                Some(n) => write!(f, "{n} "),
                None => write!(f, "- "),
            }?;
            if x == 8 {
                write!(f, "\n")?;
                if (y + 1) % 3 == 0 {
                    write!(f, "\n")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Row<'a> {
    grid: &'a Grid,
    y: u8,
}
impl<'a> Row<'a> {
    pub fn get(&self, i: u8) -> Option<&NumCell> {
        self.grid.get(i, self.y)
    }
}

#[derive(Debug)]
pub struct RowMut<'a> {
    grid: &'a mut Grid,
    y: u8,
}
impl<'a> RowMut<'a> {
    pub fn get_mut(&mut self, i: u8) -> Option<&mut NumCell> {
        self.grid.get_mut(i, self.y)
    }
    pub fn exclude(&mut self, num: Number) {
        for i in 0..9 {
            self.get_mut(i)
                .unwrap()
                .if_uncertain_mut(|c| c.exclude(num));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Column<'a> {
    grid: &'a Grid,
    x: u8,
}
impl<'a> Column<'a> {
    pub fn get(&self, i: u8) -> Option<&NumCell> {
        self.grid.get(self.x, i)
    }
}

#[derive(Debug)]
pub struct ColumnMut<'a> {
    grid: &'a mut Grid,
    x: u8,
}
impl<'a> ColumnMut<'a> {
    pub fn get_mut(&mut self, i: u8) -> Option<&mut NumCell> {
        self.grid.get_mut(self.x, i)
    }
    pub fn exclude(&mut self, num: Number) {
        for i in 0..9 {
            self.get_mut(i)
                .unwrap()
                .if_uncertain_mut(|c| c.exclude(num));
        }
    }
}

pub struct Group<'a> {
    grid: &'a Grid,
    x: u8,
    y: u8,
}
impl<'a> Group<'a> {
    pub fn get(&self, i: u8) -> Option<&NumCell> {
        let (x, y) = calc_group_xy(self.x, self.y, i)?;
        self.grid.get(x, y)
    }
}

pub struct GroupMut<'a> {
    grid: &'a mut Grid,
    x: u8,
    y: u8,
}
impl<'a> GroupMut<'a> {
    pub fn get_mut(&mut self, i: u8) -> Option<&mut NumCell> {
        let (x, y) = calc_group_xy(self.x, self.y, i)?;
        self.grid.get_mut(x, y)
    }
    pub fn exclude(&mut self, num: Number) {
        for i in 0..9 {
            self.get_mut(i)
                .unwrap()
                .if_uncertain_mut(|c| c.exclude(num));
        }
    }
}
fn calc_group_xy(group_x: u8, group_y: u8, i: u8) -> Option<(u8, u8)> {
    if i >= 9 {
        return None;
    }
    let x = group_x + (i % 3);
    let y = group_y + (i / 3);
    Some((x, y))
}

fn nth_group_xy(n: u8) -> Option<(u8, u8)> {
    if n >= 9 {
        return None;
    }
    let x = (n % 3) * 3;
    let y = (n / 3) * 3;
    Some((x, y))
}

fn group_xy_containing_xy(x: u8, y: u8) -> Option<(u8, u8)> {
    if x >= GRID_SIDE as u8 || y >= GRID_SIDE as u8 {
        return None;
    }
    let group_x = x - (x % 3);
    let group_y = y - (y % 3);
    Some((group_x, group_y))
}
