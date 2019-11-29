extern crate clap;
use clap::{App, Arg};
use std::collections::HashMap;
use std::{fs, io, u8};

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
            Arg::with_name("step")
                .short("s")
                .long("step")
                .help("Execute the program step by step")
                .takes_value(false),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();

    let program: Vec<char> = fs::read_to_string(file)
        .expect(&format!("File '{}' does not exist", file))
        .chars()
        .filter(|c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(c))
        .collect();

    let jump_table = get_jump_table(&program);

    let mut data: [u8; 30_000] = [0; 30_000];
    let mut pc = 0;
    let mut ip = 0;

    while ip < program.len() {
        let token = program[ip];

        match token {
            '>' => pc += 1,
            '<' => pc -= 1,
            '+' => data[pc] += 1,
            '-' => data[pc] -= 1,
            '.' => print!("{}", data[pc] as char),
            ',' => continue,
            '[' => {
                if data[pc] == 0 {
                    ip = match jump_table.get(&ip) {
                        Some(&jump) => jump,
                        _ => panic!(format!("No matching ] for [ at IP '{}'", ip)),
                    };
                    continue;
                }
            }
            ']' => {
                if data[pc] != 0 {
                    ip = match jump_table.get(&ip) {
                        Some(&jump) => jump,
                        _ => panic!(format!("No matching [ for ] at IP '{}'", ip)),
                    };
                    continue;
                }
            }
            _ => {}
        };

        if matches.is_present("step") {
            println!("{:?}", data[0..32].to_vec());
            println!("PC: {}", pc);
            println!("IP: {}", ip);

            let mut _input = String::new();
            io::stdin().read_line(&mut _input).expect("???");
        }

        ip += 1;
    }
}

fn get_jump_table(program: &Vec<char>) -> HashMap<usize, usize> {
    let mut table = HashMap::new();

    for (i, c) in program.iter().enumerate() {
        if *c == '[' {
            let start = i;
            let end = find_loop_end(start, program);

            table.insert(start, end + 1);
            table.insert(end, start + 1);
        }
    }

    return table;
}

fn find_loop_end(start: usize, program: &Vec<char>) -> usize {
    let mut end = start + 1;
    let mut skips = 0;

    while end < program.len() {
        if program[end] == '[' {
            skips += 1;
        } else if program[end] == ']' {
            if skips == 0 {
                return end;
            } else {
                skips -= 1;
            }
        }
        end += 1;
    }

    panic!("Unbalanced loops detected.")
}
