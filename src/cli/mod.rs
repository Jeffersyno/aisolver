use std::collections::VecDeque;

use regex::Regex;

use level::level::Level;
use level::component::Component;

pub struct Cli {
    level: Option<Level>,
    comps: Option<Vec<Component>>
}

impl Cli {
    pub fn run<I>(input_cmds: I) where I: Iterator<Item=String> {
        let mut cli = Cli { level: None, comps: None };
        let mut cmds = VecDeque::<String>::new();
        let mut rl = ::rustyline::Editor::<()>::new();
        let re_split = Regex::new(r"\s+").unwrap();

        for cmd in input_cmds { cmds.push_back(cmd); }

        loop {
            // read new command(s) from stdin
            if cmds.is_empty() {
                match rl.readline("> ") {
                    Ok(line) => {
                        rl.add_history_entry(&line);
                        for cmd in re_split.split(&line) { cmds.push_back(cmd.to_string()); }
                    },
                    Err(_) => { break; }
                }
            }

            // parse command
            else {
                let cmd = cmds.pop_front().unwrap();

                match cmd.as_str() {
                    "level" | "l" => {
                        if let Some(level_path) = cmds.pop_front() {
                            if let Ok(lvl) = Level::from_file(&level_path) {
                                cli.comps = Some(Component::all(&lvl));
                                cli.level = Some(lvl);
                            } else {
                                println!("Could not read level.");
                            }
                        } else {
                            println!("No level provided.");
                        }
                    }
                    "component" | "c" => {
                        match cli.get_component(cmds.pop_front()) {
                            Ok(comp) => println!("{}", comp),
                            Err(msg) => println!("{}", msg)
                        }
                    }
                    "print_level" | "pl" => {
                        if let Some(ref lvl) = cli.level {
                            println!("{}", lvl);
                        } else {
                            println!("No level loaded.");
                        }
                    }
                    "exit" | "quit" => { break; }
                    "help" => {
                        println!("Available commands:");
                        println!(" - level <path>");
                        println!(" - component <number>");
                        println!(" - print_level");
                        println!(" - exit / quit");
                        println!(" - help");
                    }
                    _ => { println!("Unknown command '{}'", cmd); }
                }
            }
        }
    }

    fn get_component(&self, opt_comp_nb: Option<String>) -> Result<&Component, &'static str> {
        if let Some(nb) = opt_comp_nb.and_then(|s| s.parse::<usize>().ok()) {
            if self.comps.as_ref().is_none() {
                Err("No level loaded")
            } else if nb >= self.comps.as_ref().map_or(0, |l| l.len()) {
                Err("Component number out of bounds.")
            } else {
                Ok(&self.comps.as_ref().unwrap()[nb])
            }
        } else {
            Err("Invalid component number")
        }
    }

}
