use std::fmt;
use term::{Colour as Color, Style};
use na::core::DMatrix;

use level::component::Component;
use defs::pos::Pos;

pub struct Regions {
    regions: DMatrix<i16>
}

impl Regions {
    pub fn new(comp: &Component, nb_regions: usize) -> Regions {
        let (rows, cols) = comp.size();

        let adj = comp.adjacency_matrix();
        let lap = comp.graph_laplacian(&adj);

        let eigen = ::na::linalg::SymmetricEigen::new(lap);
        let mut eigenv = eigen.eigenvalues.iter().map(|&f| f).enumerate()
            .collect::<Vec<(usize, f32)>>();

        eigenv.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let fiedler_vector = eigen.eigenvectors.column(eigenv[1].0);
        let fiedler_min = fiedler_vector.iter()
            .fold(::std::f32::INFINITY, |a, &b| if a < b { a } else { b });
        let fiedler_max = fiedler_vector.iter()
            .fold(::std::f32::NEG_INFINITY, |a, &b| if a > b { a } else { b });
        let fiedler_dst = fiedler_max - fiedler_min;

        println!("min={}, max={}", fiedler_min, fiedler_max);
        
        let mut regions = DMatrix::<i16>::from_element(rows, cols, -1);
        for row in 0..rows {
            for col in 0..cols {
                let pos = Pos::new(row as i8, col as i8);
                let index = comp.index_of(pos);

                if index == -1 { continue }

                let fiedler_val = fiedler_vector[index as usize];
                let region = (((fiedler_val - fiedler_min) / fiedler_dst) * nb_regions as f32 - 0.5) as i16;
                regions[(pos.row as usize, pos.col as usize)] = region;
            }
        }

        Regions { regions: regions }
    }
}

impl fmt::Display for Regions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = self.regions.shape();
        for row in 0..rows {
            for col in 0..cols {
                let i = self.regions[(row, col)];
                if i >= 0 {
                    let s = format!("{:>2}", i%100);
                    let c = REGION_COLORS[(i % REGION_COLORS.len() as i16) as usize];
                    write!(f, "{}", c.reverse().paint(s))?;
                } else if i == -1 {
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

const REGION_COLORS: [Color; 13] = [Color::Fixed(22), Color::Fixed(26), Color::Fixed(40),
    Color::Fixed(196), Color::Fixed(38), Color::Fixed(128), Color::Fixed(214), Color::Fixed(200),
    Color::Fixed(226), Color::Fixed(50), Color::Fixed(82), Color::Fixed(56), Color::Fixed(52)];
