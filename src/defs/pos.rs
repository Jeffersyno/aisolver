use std::ops::{Add, AddAssign, Sub, SubAssign, Neg};
use std::fmt;

pub const NULL_POS: Pos = Pos { row: -1, col: -1 };

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pos {
    pub row: i8,
    pub col: i8
}

impl Pos {
    pub fn new(row:i8, col:i8) -> Pos {
        Pos { row: row, col: col }
    }

    pub fn manhattan(self) -> i8 {
        i8::abs(self.row) + i8::abs(self.col)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        *self = Pos { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        Pos { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Pos) {
        *self = Pos { row: self.row - rhs.row, col: self.col - rhs.col }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Pos {
        Pos { row: self.row - rhs.row, col: self.col - rhs.col }
    }
}

impl Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos { row: -self.row, col: -self.col }
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.row, self.col)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    use super::Pos;

    #[test]
    fn add() { assert_eq!(Pos::new(4, 7), Pos::new(1, 3) + Pos::new(3, 4)); }

    #[test]
    fn add_assign() { let mut p = Pos::new(3, 5); p += Pos::new(1, -2); assert_eq!(Pos::new(4, 3), p); }

    #[test]
    fn sub() { assert_eq!(Pos::new(0, 0), Pos::new(3, 4) - Pos::new(3, 4)); }

    #[test]
    fn sub_assign() { let mut p = Pos::new(3, 5); p -= Pos::new(1, -2); assert_eq!(Pos::new(2, 7), p); }

    #[test]
    fn manhattan() { assert_eq!((Pos::new(3,3) - Pos::new(2,1)).manhattan(), 3); }
}
