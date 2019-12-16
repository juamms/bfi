extern crate clap;

use clap::{App, Arg};
use io::Read;
use machine::Machine;
use std::{fs, io};

mod machine;

fn main() {
    let matches = App::new("bfi")
        .version("1.1.0")
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
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Use verbose output")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("emit")
                .short("e")
                .long("emit")
                .help("Emits the intermediate representation of the program to the given FILE")
                .value_name("FILE"),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let emit = matches.value_of("emit");
    let optimise = matches.is_present("optimise");
    let step = matches.is_present("step");
    let verbose = matches.is_present("verbose");

    if verbose {
        println!("Reading raw program from '{}'...", file)
    }

    let raw_program: Vec<char> = fs::read_to_string(file)
        .expect(&format!("File '{}' does not exist", file))
        .chars()
        .filter(|c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(c))
        .collect();

    let raw_size = raw_program.len();
    let mut machine = Machine::new(30_000);

    if verbose {
        println!("Processing raw program...")
    }

    machine.load_program(raw_program, optimise);

    if verbose {
        println!("Original program size: {} instructions", raw_size);
        println!(
            "Processed program size: {} instructions",
            machine.current_program().len()
        );
    }

    if let Some(emit_file) = emit {
        if verbose {
            println!("Emiting intermediate representation to '{}'...", emit_file);
        }

        match fs::write(emit_file, machine.intermediate_representation()) {
            Ok(_) => {}
            Err(err) => {
                if verbose {
                    println!("Error writing intermediate representation file: {}", err)
                }
            }
        }
    }

    if step {
        while !machine.has_program_ended() {
            machine.step();
            machine.dump();

            let mut _input = String::new();
            io::stdin().bytes().next().expect("???").unwrap();
        }
    } else {
        machine.run();
    }
}
