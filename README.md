# bfi
An experimental, optimising Brainfuck interpreter, written in Rust.

## Usage

```    
bfi [FLAGS] [OPTIONS] --file <FILE>

FLAGS:
    -h, --help        Prints help information
    -o, --optimise    Optimise the program before running
    -s, --step        Execute the program step by step
    -V, --version     Prints version information
    -v, --verbose     Use verbose output

OPTIONS:
    -e, --emit <FILE>    Emits the intermediate representation of the program to the given FILE
    -f, --file <FILE>    The file to execute
```
