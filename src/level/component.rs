use std::fmt;
use std::ops::Index;

use defs::grid::Grid;
use defs::pos::Pos;
use defs::dir::DIRS;

use super::level::Level;
use super::item::Item;

#[derive(Debug, Clone)]
pub struct Component {
    grid: Grid<Item>
}

impl Component {
    pub fn new(grid: Grid<Item>) -> Component {
        Component { grid: grid }
    }

    pub fn all(level: &Level) -> Vec<Component> {
        let (rowsu, colsu) = level.size();
        let (rows, cols) = (rowsu as i8, colsu as i8);
        let mut done = Grid::<bool>::new(rowsu, colsu);
        let mut comp = Grid::<Item>::new_fill(rowsu, colsu, Item::wall());
        let mut comps = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);

                if done[pos] { continue; }
                if level[pos].is_wall() { done[pos] = true; continue; }

                let mut contains_goal = false;
                let mut queue = vec!(pos);
                while !queue.is_empty() {
                    let cell = queue.pop().unwrap();
                    comp[cell] = level[cell];
                    done[cell] = true;
                    contains_goal |= level[cell].is_goal();

                    for p in DIRS.iter().map(|&d| cell + d) {
                        if 0 > p.row || p.row >= rows || 0 > p.col || p.col >= cols { continue; }
                        if !level[p].is_wall() && !done[p] { queue.push(p); }
                    }
                }

                if contains_goal {
                    comps.push(Component::new(comp.clone()));
                }
                comp.fill(Item::wall());
            }
        }

        comps
    }

    pub fn size(&self) -> (usize, usize) {
        self.grid.size()
    }
}

impl Index<Pos> for Component {
    type Output = Item;
    fn index(&self, pos: Pos) -> &Item {
        &self.grid[pos]
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = self.size();
        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);
                write!(f, "{}", self[pos])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
