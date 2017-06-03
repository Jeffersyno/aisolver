
use super::pos::Pos;

pub type Dir = Pos;

pub const NORTH: Dir = Dir {row:-1, col: 0};
pub const EAST:  Dir = Dir {row: 0, col: 1};
pub const SOUTH: Dir = Dir {row: 1, col: 0};
pub const WEST:  Dir = Dir {row: 0, col:-1};

pub const DIRS: [Dir; 4] = [NORTH, EAST, SOUTH, WEST];

pub fn name(dir: Dir) -> &'static str {
    match dir {
        NORTH => "North",
        EAST  => "East",
        SOUTH => "South",
        WEST  => "West",
        _     => "?Dir?"
    }
}
