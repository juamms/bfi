extern crate structopt;

use io::Read;
use machine::Machine;
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

mod machine;

#[derive(StructOpt)]
#[structopt(
    name = "bfi",
    version = "1.1.0",
    about = "An experimental Brainfuck interpreter"
)]
struct Opt {
    #[structopt(short, long, parse(from_os_str), help = "The file to execute")]
    file: PathBuf,

    #[structopt(short, long, help = "Optimise the program before running")]
    optimise: bool,

    #[structopt(short, long, help = "Execute the program step by step")]
    step: bool,

    #[structopt(short, long, help = "Use verbose output")]
    verbose: bool,

    #[structopt(
        short,
        long,
        help = "Emits the intermediate representation of the program to the given FILE"
    )]
    emit: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let file = &opt.file;

    if opt.verbose {
        println!("Reading raw program from {:?}...", file)
    }

    let raw_program: Vec<char> = match fs::read_to_string(file) {
        Ok(content) => content
            .chars()
            .filter(|c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(c))
            .collect(),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let raw_size = raw_program.len();
    let mut machine = Machine::new(30_000);

    if opt.verbose {
        println!("Processing raw program...")
    }

    machine.load_program(raw_program, opt.optimise);

    if opt.verbose {
        println!("Original program size: {} instructions", raw_size);
        println!(
            "Processed program size: {} instructions",
            machine.current_program().len()
        );
    }

    if let Some(emit_file) = opt.emit {
        if opt.verbose {
            println!("Emiting intermediate representation to {:?}...", emit_file);
        }

        match fs::write(emit_file, machine.intermediate_representation()) {
            Ok(_) => {}
            Err(err) => {
                if opt.verbose {
                    println!("Error writing intermediate representation file: {}", err)
                }
            }
        }
    }

    if opt.step {
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
