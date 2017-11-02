extern crate regex;
extern crate ansi_term as term;
extern crate rustyline;
extern crate nalgebra as na;

mod cli;
mod defs;
mod level;

fn main() {
    cli::Cli::run(std::env::args().skip(1));
}
