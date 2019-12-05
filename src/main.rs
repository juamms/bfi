extern crate clap;

use clap::{App, Arg};
use machine::Machine;
use std::{fs, io};

mod machine;

fn main() {
    let matches = App::new("bfi")
        .version("0.1.0")
        .about("An experimental Brainfuck interpreter")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("The file to execute")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("optimise")
                .short("o")
                .long("optimise")
                .help("Optimise the program before running")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("step")
                .short("s")
                .long("step")
                .help("Execute the program step by step")
                .takes_value(false),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let optimise = matches.is_present("optimise");
    let step = matches.is_present("step");

    let raw_program: Vec<char> = fs::read_to_string(file)
        .expect(&format!("File '{}' does not exist", file))
        .chars()
        .filter(|c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(c))
        .collect();

    let mut machine = Machine::new(30_000);
    machine.load_program(raw_program, optimise);

    if step {
        while !machine.has_program_ended() {
            machine.step();
            machine.dump();

            let mut _input = String::new();
            io::stdin().read_line(&mut _input).expect("???");
        }
    } else {
        machine.run();
    }
}
