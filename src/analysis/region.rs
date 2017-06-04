use std::fmt;
use term::{Colour as Color, Style};

use defs::dir::DIRS;
use defs::grid::Grid;
use defs::pos::Pos;
use level::component::Component;

type RegionId = i16;

const NULL_REGION_ID: RegionId = -1;
const UNDEF_REGION_ID: RegionId = -2;

pub struct Regions {
    grid: Grid<RegionId>
}

impl Regions {
    pub fn new(comp: &Component) -> Regions {
        let (rows, cols) = comp.size();
        let weights = Regions::cell_weights(comp);
        let mut candidates = Vec::new();
        let mut grid = Grid::new_fill(rows, cols, UNDEF_REGION_ID);

        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);
                if !comp[pos].is_wall() { candidates.push(pos); }
                else                    { grid[pos] = NULL_REGION_ID; }
            }
        }

        candidates.sort_by(|&p, &q| i16::cmp(&weights[q], &weights[p]));

        println!("{}", weights);
        println!("{:?}", candidates);

        for cand in candidates {
        }

        Regions { grid: grid }
    }

    pub fn cell_weights(comp: &Component) -> Grid<i16> {
        let (rows, cols) = comp.size();
        let mut weights = Grid::new_fill(rows, cols, 1<<12);
        let mut updated = true;

        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);
                if comp[pos].is_wall() { weights[pos] = 0; }
            }
        }

        while updated {
            updated = false;
            for row in 1..rows-1 {
                for col in 1..cols-1 {
                    let pos = Pos::new(row as i8, col as i8);
                    for n in DIRS.iter().map(|&d| pos + d) {
                        if weights[pos] > weights[n] + 1 {
                            updated = true;
                            weights[pos] = weights[n] + 1;
                        }
                    }
                }
            }
        }

        weights
    }
}

impl fmt::Display for Regions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = self.grid.size();
        for row in 0..rows {
            for col in 0..cols {
                let i = self.grid[(row, col)];
                if i >= 0 {
                    let s = format!("{:>2}", i%100);
                    let c = REGION_COLORS[(i % REGION_COLORS.len() as RegionId) as usize];
                    write!(f, "{}", c.reverse().paint(s))?;
                } else if i == NULL_REGION_ID {
                    write!(f, "{}", Style::new().dimmed().reverse().paint("  "))?;
                } else {
                    write!(f, "{}", Color::Red.bold().reverse().paint("??"))?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

const CORNERS: [Pos; 4] = [Pos { row: 0, col: 0 }, Pos { row: 0, col: 1 },
                           Pos { row: 1, col: 0 }, Pos { row: 1, col: 1 }];

const REGION_COLORS: [Color; 13] = [Color::Fixed(22), Color::Fixed(26), Color::Fixed(40),
    Color::Fixed(196), Color::Fixed(38), Color::Fixed(128), Color::Fixed(214), Color::Fixed(200),
    Color::Fixed(226), Color::Fixed(50), Color::Fixed(82), Color::Fixed(56), Color::Fixed(52)];
