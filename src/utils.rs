use std::{fs, io::Write, path::PathBuf, process::{Command, Stdio}};

const TAB: &str = "     ";
const HEADERS: [&str; 4] = [
    "#include <stdio.h>\n\nint main() {\n",
    "     unsigned char tape[30000] = {0};\n",
    "     unsigned char* ptr = tape;\n\n",
    "     return 0;\n}"
];

#[derive(Clone)]
pub struct Program {
    pub compiler_flag: Option<String>,

    pub input: PathBuf,
    pub brainfuck: String,
    pub output: PathBuf,
    pub c: String
}

impl Program {
    pub fn new() -> Self {
        let compiler_flag = Some(String::from(""));
        let input = PathBuf::from(".");
        let brainfuck = String::new();
        let output = PathBuf::from(".");
        let c = String::new();

        Self {
            compiler_flag,
            input,
            brainfuck,
            output,
            c
        }
    }

    pub fn handle_arguments(&mut self, args: Vec<String>) {
        match args.len() { // [program] <>.bf <>
            3 => {
                match self.check_arguments(args, 2) {
                    Ok(_) => {}
                    _ => std::process::exit(1)
                }
            }
            4 => {
                if args.get(2).unwrap() != "-o" { // [program] <>.bf -o <>
                    eprintln!("\x1b[1;31m[] ERROR\x1b[0m: Invalid flag! at: \x1b[1;31m\"{}\" \x1b[0m", args.get(2).unwrap_or(&"[Something has gone terribly wrong.]".to_string()));
                    std::process::exit(1);
                } else {
                    match self.check_arguments(args, 3) {
                        Ok(_) => {}
                        _ => std::process::exit(1)
                    }
                }
            }
            5 => { // [program] <>.bf -o <> -O3
                if args.get(2).unwrap() != "-o" { // [program] <>.bf -o <>.c
                    eprintln!("\x1b[1;31m[] ERROR\x1b[0m: Invalid flag! at: \x1b[1;31m\"{}\" \x1b[0m", args.get(2).unwrap_or(&"[Something has gone terribly wrong.]".to_string()));
                    std::process::exit(1);
                } else {
                    match self.check_arguments(args.clone(), 3) {
                        Ok(_) => {
                            match args.get(4) {
                                Some(flag) => {
                                    self.compiler_flag = Some(flag.to_string());
                                }
                                None => {}
                            }
                        }
                        
                        _ => std::process::exit(1)
                    }
                }
            }
            _ => {
                eprintln!("\x1b[1;31m[] ERROR\x1b[0m: Bad arguments!\n\x1b[1;31m[] (arguments must be > 2)\x1b[0m");
                std::process::exit(1);
            }
        }
    }
    fn check_arguments(&mut self, args: Vec<String>, num: usize) -> std::io::Result<()>{
        if args.get(1).unwrap().contains('.') {
            let parts_i: Vec<&str> =  args.get(1).unwrap().split('.').collect();
            if parts_i[1] != "bf" && parts_i[1] != "b" {
                eprintln!("\x1b[1;31mERROR\x1b[0m: Input file ends in: \x1b[1;31m'{}'\x1b[0m - is it a Brainfuck file? (.bf / .b)", parts_i.get(1).unwrap_or(&"[NO FILE EXTENSION]"));
                std::process::exit(1)
            } else { 
                self.input = PathBuf::from(args.get(1).expect("NO INPUT FILE"));
            }
            self.output = PathBuf::from(args.get(num).expect("God dislikes you."));
        } 
        else {
            eprintln!("\x1b[1;31m[] ERROR\x1b[0m: Bad arguments!\n[1;31m[] File ending in .bf not found!\x1b[0m");
            std::process::exit(1);
        }
        Ok(())
    }

    pub fn read_from_bf(&mut self) {
        if fs::metadata(self.input.clone()).is_ok() {
            self.brainfuck = fs::read_to_string(self.input.clone()).expect("\x1b[1;31m[] Something has gone terribly wrong.\x1b[0m");

        } else {
            eprintln!("\x1b[1;31m[] ERROR\x1b[0m: NO FILE!\n[1;31m[] Brainfuck file {:?} does not exist!\x1b[0m", self.input.clone());
            std::process::exit(1);
        }
    }

    pub fn transpile_to_c(&mut self) {
        for i in 0..=2 {
            self.c.push_str(HEADERS[i])
        }
    
        for instruction in self.brainfuck.chars() {
            match instruction {
                '>' => self.c.push_str(&format!("{}++ptr;\n", TAB)),
                '<' => self.c.push_str(&format!("{}--ptr;\n", TAB)),
                '+' => self.c.push_str(&format!("{}++*ptr;\n", TAB)),
                '-' => self.c.push_str(&format!("{}--*ptr;\n", TAB)), 
                '.' => self.c.push_str(&format!("{}putchar(*ptr);\n", TAB)),
                ',' => self.c.push_str(&format!("{}*ptr = getchar();\n", TAB)), 
                '[' => self.c.push_str(&format!("{}while (*ptr) {{\n", TAB)), 
                ']' => self.c.push_str(&format!("{}}}\n", TAB)),
                _ => {}
            }
        }
    
        self.c.push_str(HEADERS[3]);
    }
    
    pub fn compile_c(&mut self) {
        let mut gcc_process = Command::new("gcc").args(&["-o", self.output.to_str().unwrap(), "-x", "c", &self.
            compiler_flag
            .clone()
            .unwrap_or(""
            .to_string()), "-"])
            .stdin(Stdio::piped())
            .spawn().expect("GCC ERROR");
        if let Some(mut stdin) = gcc_process.stdin.take() {
            stdin.write_all(self.c.as_bytes()).unwrap();
        }
        let output = gcc_process.wait().unwrap();
        if output.success() {
            println!("\x1b[1;32m[] BINARY EXECUTABLE CREATED\x1b[0m");
            std::process::exit(0);
        } else {
            eprintln!("\x1b[1;31m[] GCC FAILED TO COMPILE\x1b[0m");
            std::process::exit(1);
        }
    }
}