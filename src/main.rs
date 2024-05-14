mod utils;

use std::env;
use std::fs::File;
use std::io::{Write, Read};

use utils::Program;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut program: Program = Program::new();
    program.handle_arguments(args);
    program.read_from_bf();     // this feels like writing python.
    program.transpile_to_c();
    program.compile_c();
}