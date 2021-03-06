use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use super::pos::Pos;

#[derive(Debug)]
pub struct Grid<T> {
    rows: usize,
    data: Vec<T>
}

impl <T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Grid<T> where T: Default {
        Self::new_fn(rows, cols, || Default::default())
    }

    pub fn new_fill(rows: usize, cols: usize, value: T) -> Grid<T> where T: Copy {
        Self::new_fn(rows, cols, || value)
    }

    pub fn new_fn<F: Fn()->T>(rows: usize, cols: usize, f: F) -> Grid<T> {
        let cap = rows * cols;
        let mut data = Vec::<T>::with_capacity(cap);

        for _ in 0..cap { data.push(f()); }

        Grid {
            rows: rows,
            data: data
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>            { self.data.iter() }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> { self.data.iter_mut() }

    pub fn size(&self) -> (usize, usize) {
        let cols = self.data.len() / self.rows;
        (self.rows, cols)
    }

    pub fn fill(&mut self, val: T) where T: Copy {
        for i in self.iter_mut() { *i = val; }
    }

    pub fn linear_index_of_pos(&self, pos: Pos) -> usize {
        self.linear_index_of(pos.row as usize, pos.col as usize)
    }

    pub fn linear_index_of<I>(&self, row: I, col: I) -> usize where I: Into<usize> {
        let row = Into::<usize>::into(row);
        let col = Into::<usize>::into(col);

        let size = self.size();

        if row >= size.0 { panic!("grid row out of bounds"); }
        if col >= size.1 { panic!("grid col out of bounds"); }

        let index = row + self.rows * col;

        index
    }
}

impl <T> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, pos: Pos) -> &T {
        Index::index(self, (pos.row as u8, pos.col as u8))
    }
}

impl <I, T> Index<(I,I)> for Grid<T> where I: Into<usize> {
    type Output = T;
    fn index(&self, tuple: (I, I)) -> &T {
        let index = self.linear_index_of(tuple.0, tuple.1);
        &self.data[index]
    }
}

impl <T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut T {
        IndexMut::index_mut(self, (pos.row as u8, pos.col as u8))
    }
}

impl <I, T> IndexMut<(I,I)> for Grid<T> where I: Into<usize> {
    fn index_mut(&mut self, tuple: (I, I)) -> &mut T {
        let index = self.linear_index_of(tuple.0, tuple.1);
        &mut self.data[index]
    }
}

impl <T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Grid<T> {
        Grid {
            rows: self.rows,
            data: self.data.clone()
        }
    }
}

impl <T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_len = self.data.iter().map(|i| format!("{}", i).len()).max().unwrap_or(3);

        let (rows, cols) = self.size();
        write!(f, "Grid({}×{})[\n", rows, cols)?;

        for row in 0..rows {
            for col in 0..cols {
                write!(f, "{:width$}", self[(row, col)], width=max_len+1)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Grid;
    use super::super::pos::Pos;

    #[test]
    fn test_grid() {
        let mut grid = Grid::<i8>::new(3, 3);
        grid[(1u8, 2u8)] = 3i8;
        assert_eq!(grid[(Pos::new(1, 2))], 3i8)
    }

    #[test]
    fn test_grid_size() {
        let grid = Grid::<i8>::new(3, 4);
        assert_eq!(grid.size(), (3, 4))
    }
}
