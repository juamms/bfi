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

## Implementation notes

The interpreter virtual machine currently supports the following instructions:

```rust
enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    Clear,
    LoopStart(usize),
    LoopEnd(usize),
    Read,
    Write,
}
```


With the exception of `Clear`, all of the instructions are matched 1-to-1 to Brainfuck instructions, with `1` as the value for `MoveRight`, `MoveLeft`, `Increment` and `Decrement` when running the program as-is. After parsing and optimising, the interpreter calculates jump positions for all `LoopStart` and `LoopEnd` matching tokens so that jumps are processed immediately. The interpreter panics if it finds unbalanced loop tokens.

## Current optimisations

By enabling the optimise flag, the interpreter will apply the following optimisations to the program:

* Consecutive `>`, `<`, `+`, `-` tokens are bundled together as a single instruction to the virtual machine (e.g. `>>>>` becomes `MoveRight(4)`);
* When the interpreter finds the token sequence `[-]`, it substitutes that loop (which can have at most 255 iterations) to a single `Clear` instruction;

As a reference, here's a few numbers when running some example programs unoptimised vs optimised with the `time` command on a mid-2014 MacBook Pro:

Program | Execution Time (u) | Execution Time (o) | Instructions (u) | Instructions (o)
------------ | ------------- | ------------ | ------------- | -------------
Mandelbrot by Erik Bosman | 47.92s | 13.28s | 11451 | 3867
Mandelbrot (huge) by Erik Bosman | 260.68s | 69.80s | 11467 | 3867
Towers of Hanoi by Clifford Wolf | 24.53s | 1.12s | 53884 | 14863
