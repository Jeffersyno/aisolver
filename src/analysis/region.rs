use std::fmt;
use term::{Colour as Color, Style};

use defs::pos::Pos;
use defs::grid::Grid;
use level::component::Component;

pub type RegionId = i16;
pub const NULL_REGION_ID: RegionId = -1;

#[derive(Debug)]
pub struct Regions {
    regions: Vec<Vec<Pos>>,
    grid: Grid<RegionId>
}

impl Regions {

    pub fn new(comp: &Component) -> Regions {
        Regions::init(comp)
    }

    fn init(comp: &Component) -> Regions {
        let (rows, cols) = comp.size();
        let mut grid = Grid::new_fill(rows, cols, NULL_REGION_ID);
        let mut regions = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);
                if !comp[pos].is_wall() {
                    grid[pos] = regions.len() as RegionId;
                    regions.push(vec![pos]);
                } else {
                    grid[pos] = NULL_REGION_ID;
                }
            }
        }

        Regions { regions: regions, grid: grid }
    }
}

const REGION_COLORS: [Color; 13] = [Color::Fixed(22), Color::Fixed(26), Color::Fixed(40),
    Color::Fixed(196), Color::Fixed(38), Color::Fixed(128), Color::Fixed(214), Color::Fixed(200),
    Color::Fixed(226), Color::Fixed(50), Color::Fixed(82), Color::Fixed(56), Color::Fixed(52)];

impl fmt::Display for Regions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of regions: {}\n", self.regions.len())?;

        let (rows, cols) = self.grid.size();
        for row in 0..rows {
            for col in 0..cols {
                let i = self.grid[(row, col)];
                if i >= 0 {
                    let s = format!("{:>2}", i%100);
                    let c = REGION_COLORS[(i % REGION_COLORS.len() as RegionId) as usize];
                    write!(f, "{}", c.reverse().paint(s))?;
                } else {
                    write!(f, "{}", Style::new().dimmed().reverse().paint("  "))?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
