use crate::number::{Number, ParseNumberError};
use std::collections::HashSet;

// #[derive(Debug, Clone)]
// pub enum NumCellState {
//     Collapsed(u8),
//     Uncertain(UncertainCell), //Undecided(HashSet<u8>),
// }

#[derive(Debug, Clone)]
pub enum NumCell {
    Collapsed(Number),
    Uncertain(UncertainCell),
}

#[derive(Debug, Clone)]
pub struct UncertainCell {
    set: HashSet<Number>,
}

impl NumCell {
    pub fn new() -> Self {
        Self::Uncertain(UncertainCell::new())
    }
    pub fn is_collapsed(&self) -> bool {
        matches!(self, Self::Collapsed(_))
    }
    pub fn uncertain(&self) -> Option<&UncertainCell> {
        match self {
            Self::Uncertain(c) => Some(c),
            _ => None,
        }
    }
    pub fn uncertain_mut(&mut self) -> Option<&mut UncertainCell> {
        match self {
            Self::Uncertain(c) => Some(c),
            _ => None,
        }
    }
    pub fn number(&self) -> Option<Number> {
        match self {
            Self::Collapsed(n) => Some(*n),
            _ => None,
        }
    }
    pub fn if_uncertain<F, T>(&self, mut f: F) -> Option<T>
    where
        F: FnMut(&UncertainCell) -> T,
    {
        if let Self::Uncertain(c) = self {
            Some(f(c))
        } else {
            None
        }
    }
    pub fn if_uncertain_mut<F, T>(&mut self, mut f: F) -> Option<T>
    where
        F: FnMut(&mut UncertainCell) -> T,
    {
        if let Self::Uncertain(c) = self {
            Some(f(c))
        } else {
            None
        }
    }
}
impl UncertainCell {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn entropy(&self) -> usize {
        self.set.len()
    }
    pub fn exclude(&mut self, num: Number) {
        self.set.remove(&num);
    }
    pub fn try_collapse(&self) -> Option<Number> {
        if self.entropy() != 1 {
            return None;
        }
        self.set.iter().next().copied()
    }
    pub fn some_element(&self) -> Option<Number> {
        self.set.iter().next().copied()
    }
    pub fn collapse(&self) -> Number {
        if self.entropy() != 1 {
            panic!("tried to collapse an uncertain cell");
        }
        *self.set.iter().next().unwrap()
    }
}
impl Default for UncertainCell {
    fn default() -> Self {
        let mut set = HashSet::with_capacity(9);
        for i in 1..=9 {
            set.insert(Number::new(i));
        }
        Self { set }
    }
}

impl std::str::FromStr for NumCell {
    type Err = ParseNumberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" | "-" | "." | "," => Ok(Self::new()),
            s => Ok(Self::Collapsed(s.parse()?)),
        }
    }
}
