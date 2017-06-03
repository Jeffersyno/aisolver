extern crate regex;
extern crate ansi_term as term;
extern crate rustyline;

mod cli;
mod defs;
mod level;
mod analysis;

fn main() {
    cli::Cli::run(std::env::args().skip(1));
}
