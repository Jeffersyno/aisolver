use std::fmt;
use std::ops::Index;

use na::core::DMatrix;

use defs::grid::Grid;
use defs::pos::Pos;
use defs::dir::DIRS;

use super::level::Level;
use super::item::Item;

#[derive(Debug, Clone)]
pub struct Component {
    grid: Grid<Item>,               // grid of the component, walls in non-reachable cells
    pos_to_index: DMatrix<i16>,     // look up the cell index of a position
    index_to_pos: Vec<Pos>          // look up a position given a cell index
}

impl Component {

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
                    let component = Component::new(&comp);
                    comps.push(component);
                }
                comp.fill(Item::wall());
            }
        }

        comps
    }

    pub fn new(grid: &Grid<Item>) -> Component {
        let size = grid.size();
        let mut pos_to_index = DMatrix::<i16>::from_element(size.0, size.1, -1);
        let mut index_to_pos = Vec::new();

        for col in 0..size.1 {
            for row in 0..size.0 {
                let pos = Pos::new(row as i8, col as i8);

                if !grid[pos].is_wall() {
                    pos_to_index[(row, col)] = index_to_pos.len() as i16;
                    index_to_pos.push(pos);
                }
            }
        }

        Component {
            grid: grid.clone(),
            pos_to_index: pos_to_index,
            index_to_pos: index_to_pos
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.grid.size()
    }

    pub fn index_of(&self, pos: Pos) -> i16 {
        self.pos_to_index[(pos.row as usize, pos.col as usize)]
    }

    pub fn pos_of(&self, index: i16) -> Pos {
        self.index_to_pos[index as usize]
    }

    pub fn nb_free_cells(&self) -> usize {
        self.index_to_pos.len()
    }

    pub fn adjacency_matrix(&self) -> DMatrix<i8> {
        let n = self.nb_free_cells();
        let mut matrix = DMatrix::<i8>::zeros(n, n);

        for i in 0..n {
            let current = self.pos_of(i as i16);

            for &d in &DIRS {
                let neighbor = current + d;

                if !self[neighbor].is_wall() {
                    let neighbor_index = self.index_of(neighbor);
                    matrix[(i, neighbor_index as usize)] += 1;
                }
            }
        }

        matrix
    }

    /// (D - A), with
    ///     D the diagonal matrix containing the degrees of the nodes
    ///     A the adjacency matrix
    pub fn graph_laplacian(&self, adj: &DMatrix<i8>) -> DMatrix<f32> {
        let n = self.nb_free_cells();
        let mut laplacian = adj.map(|n| -n as f32);

        for i in 0..n {
            let current = self.pos_of(i as i16);
            let degree = (&DIRS).iter().map(|&d| {
                if !self[current + d].is_wall() { 1f32 }
                else { 0f32 }
            }).sum();

            laplacian[(i, i)] = degree;
        }

        laplacian
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

        let adj = self.adjacency_matrix();

        write!(f, "{}", self.pos_to_index)?;
        write!(f, "{}", adj)?;
        write!(f, "{}", self.graph_laplacian(&adj))?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use level::level::Level;

    #[test]
    fn test_pos_index() {
        let level = Level::from_file("levels/rbts.lvl").unwrap();
        let comps = Component::all(&level);
        let comp = &comps[0];

        let (rows, cols) = comp.size();

        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);

                let pos_index = comp.index_of(pos);

                if pos_index == -1 {
                    assert!(comp[pos].is_wall());
                } else {
                    let index_pos = comp.pos_of(pos_index);
                    assert!(index_pos == pos);
                }
            }
        }
    }
}
