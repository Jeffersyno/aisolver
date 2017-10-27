use std::collections::HashMap;
use std::fmt;
use std::ops::Index;

use regex::Regex;

use defs::grid::Grid;
use defs::pos::Pos;
use super::item::{Item, Color};

pub struct Level {
    grid: Grid<Item>
}

impl Level {
    pub fn new(level_str: &str) -> Level {
        use std::cmp::max;

        let re_split = Regex::new(r"\s*[\r\n]+").unwrap();
        let re_color = Regex::new(r"^(\w+)\s*:\s*(([0-9a-zA-Z],?\s*)+)").unwrap();
        let re_comma = Regex::new(r",\s*").unwrap();

        let mut rows = 0;
        let mut cols = 0;

        for line in re_split.split(level_str) {
            if !re_color.is_match(line) { rows+=1; cols = max(cols, line.len()); }
        }

        let mut color_map = HashMap::<u8, Color>::new();
        let mut grid = Grid::<Item>::new(rows, cols);
        let mut row = 0;

        for line in re_split.split(level_str) {
            if let Some(captures) = re_color.captures(line) {
                let color = captures.get(1).unwrap().as_str();
                let items = captures.get(2).unwrap().as_str();

                for item_str in re_comma.split(items) {
                    let idl = item_str.to_lowercase().as_bytes()[0];
                    let idu = item_str.to_uppercase().as_bytes()[0];
                    let color = Color::find_color_by_str(color).expect("invalid color");
                    color_map.insert(idl, color);
                    color_map.insert(idu, color);
                }
            } else {
                for (col, chr) in line.as_bytes().into_iter().enumerate() {
                    let color = *color_map.get(chr).unwrap_or(&Color::Blue);
                    grid[(row, col)] = Item::new(*chr, color);
                }
                row += 1;
            }
        }

        let level = Level { grid: grid };
        level
    }

    pub fn from_file(path: &str) -> Result<Level, &'static str> {
        use std::fs::File;
        use std::io::Read;

        let mut s = String::new();

        match File::open(path).and_then(|mut f| f.read_to_string(&mut s)) {
            Ok(_)  => Ok(Level::new(&s)),
            Err(_) => Err("cannot read file")
        }
    }

    pub fn size(&self) -> (usize, usize) { self.grid.size() }
}


impl Index<Pos> for Level {
    type Output = Item;
    fn index(&self, pos: Pos) -> &Item {
        &self.grid[pos]
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = self.size();
        write!(f, "Level [{}×{}].\n", rows, cols)?;
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

#[cfg(test)]
mod test {
    use super::*;
}
