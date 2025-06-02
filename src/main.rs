use std::process::exit;
use std::process::Command;
use std::path::Path;
use std::io::Write;

use err::Diagnostic;
mod tokenizer;
mod parser;
mod compiler;
mod typing;
mod err;

const HELP: &str = "sabbahc - cli for the Sabbah language

Usage:
  sabbahc <INPUT> [OPTIONS]

Options:
  -o, --output <file>     Specify output filename
  -s, --asm               Compile only; do not assemble or link
  -b, --object            Compile and assemble; do not link
  -m, --mode <mode>       Manually specify the output mode
  -v, --version           Show version information and exit
  -h, --help              Show this help message and exit
  -f, --force             Ignore nonfatal errors

Description:
  sabbahc is a command-line compiler for the Sabbah programming language. It
  supports compiling to assembly, object code, and fully linked executables.

Examples:
  sabbahc main.sbb         # Compile and link (default output: <filename>.out)
  sabbahc main.sbb -o prog # Compile and link, output to 'prog'
  sabbahc main.sbb -s      # Compile to Assembly only
";

#[derive(PartialEq, Debug)]
enum OutputMode {
    Assembly,
    Object,
    BinaryExecutable,
}

#[derive(Debug)]
enum EarlyExit {Version, Help}

#[derive(Debug)]
struct CLIInstructions {
    input: String,
    output: String,
    mode: OutputMode,
    exit_early: Option<EarlyExit>,
    force: bool,
}

impl CLIInstructions {
    pub fn from(args: Vec<String>) -> CLIInstructions {
        let mut input: String = args[1].clone();
        let mut output: String = String::new();
        let mut mode: OutputMode = OutputMode::BinaryExecutable;
        let mut exit_early: Option<EarlyExit> = None;
        let mut force: bool = false;
        let mut i = 1 /* skip commmand */;
        let mut input_set: bool = false;
        while i < args.len() {
            match args[i].as_str() {
                        "-h" | "--help" => {
                            exit_early = Some(EarlyExit::Help);
                        }
                        "-f" | "--force" => {
                            force = true;
                        }
                        "-v" | "--version" => {
                            exit_early = Some(EarlyExit::Version);
                        }
                        "-o" | "--output" => {
                            if i + 1 < args.len() {
                                output = args[i + 1].clone();
                                i += 1;
                            } else {
                                println!("ERROR: -o flag requires an argument");
                                exit(3);
                            }
                        }
                        "-s" | "--asm" => {
                            mode = OutputMode::Assembly;
                        }
                        "-b" | "--object" => {
                            mode = OutputMode::Object;
                        }
                        "-m" | "--mode" => {
                            if i + 1 < args.len() {
                                match args[i + 1].as_str() {
                                    "asm" | "assembly" | "s" => mode = OutputMode::Assembly,
                                    "obj" | "object" => mode = OutputMode::Object,
                                    "bin" | "binary" => mode = OutputMode::BinaryExecutable,
                                    _ => {println!("ERROR: Unrecognized mode: {}", args[i + 1]); exit(4);}
                                }
                                i += 1;
                            } else {
                                println!("ERROR: -m flag requires an argument");
                                exit(5);
                            }
                        }
                    _ => {
                        if !input_set {
                            input = args[i].clone();
                            input_set = true;
                        }
                        else if args[i].starts_with("-") {
                            println!("ERROR: Unrecognized flag/option: {}", args[i]);
                            exit(2);
                        }
                        else {
                            println!("ERROR: Unexpected argument: {}", args[i]);
                        }
                    }
            }
            i += 1;
        }
    if input.is_empty() {
        println!("ERROR: No input file specified");
        exit(6);
    }
    if output.is_empty() { // Generate output file name from input
        let input_path = Path::new(&input);
        let mut output_file_name = input_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("output")
            .to_string();
        if output_file_name.ends_with(".sbb") {
            output_file_name = output_file_name[0..output_file_name.len()-4].to_string();
        }
        match mode {
            OutputMode::Assembly => output = format!("{}.s", output_file_name),
            OutputMode::Object => output = format!("{}.o", output_file_name),
            OutputMode::BinaryExecutable => output = format!("{}", output_file_name),
        }
    }
    return CLIInstructions {
        input: input,
        output: output,
        mode: mode,
        exit_early: exit_early,
        force: force,
    }
    }
}

const VERSION: &str = "0.0.1";

fn find_empty_temp_filename() -> String {
    let mut counter = 0;
    loop {
        let filename = format!("out{}.tmp", counter);
        if !Path::new(&filename).exists() {
            return filename;
        }
        counter += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print!("{}", HELP);
        exit(1);
    }
    let instructions: CLIInstructions = CLIInstructions::from(args);
    if let Some(exit_early) = instructions.exit_early {
        match exit_early {
            EarlyExit::Version => {
                println!("sabbahc version {}", VERSION);
                exit(0);
            }
            EarlyExit::Help => {
                println!("{}", HELP);
                exit(0);
            }
        }
    }
    /* Check file requirements */
    if !instructions.force {
        let input = &instructions.input.clone();
        let output = &instructions.output.clone();
        if !std::fs::exists(input.clone()).unwrap() {
            println!("ERROR: Input file {} does not exist", input.as_str());
            exit(8);
        }
        if std::fs::exists(output).unwrap() {
            println!("ERROR: Output file {} already exists", output.as_str());
            exit(9);
        }
    }
    /* */
    let input = std::fs::read_to_string(instructions.input.clone()).expect("Failed to read input file");
    let mut tokenizer = tokenizer::Tokenizer::new(&input, instructions.input.clone());
    let tokenized: Vec<tokenizer::Token> = tokenizer.tokenize();
    let mut parser = parser::Parser::new(tokenized, instructions.input.clone());
    let parsed: Vec<parser::Ast> = parser.parse(&input);
    let mut compiler = compiler::Compiler::new(parsed);
    let compiled: String = compiler.compile();

    match instructions.mode {
        OutputMode::Assembly => {
            let mut output = std::fs::File::create(instructions.output.clone()).unwrap();
            output.write_all(compiled.as_bytes()).unwrap();
            println!("Successfully compiled to file \"{}\"", instructions.output.clone());
        }
        OutputMode::Object => {
            let asm_output = find_empty_temp_filename();
            let mut asm_file = std::fs::File::create(asm_output.clone()).unwrap();
            asm_file.write_all(compiled.as_bytes()).unwrap();
            let object_output = instructions.output.clone();
            let assembler_output = Command::new("gcc")
                .arg("-o")
                .arg(format!("{}", instructions.output.clone()))
                .arg("-x")
                .arg("assembler")
                .arg("-nostdlib")
                .arg("-static")
                .arg("-c")
                .arg(format!("{}", asm_output))
                .output()
                .expect("Failed to execute assembler!");
            if assembler_output.status.success() {
                println!("Successfully assembled to file \"{}\"", object_output);
            } else {
                println!("Assembler failed with error: {}", String::from_utf8_lossy(&assembler_output.stderr));
                exit(11);
            }
            std::fs::remove_file(asm_output).expect("Failed to remove temporary asm file");
        }
        OutputMode::BinaryExecutable => {
            let asm_output = find_empty_temp_filename();
            let mut asm_file = std::fs::File::create(asm_output.clone()).unwrap();
            asm_file.write_all(compiled.as_bytes()).unwrap();
            let assembler_output = Command::new("gcc")
                .arg("-o")
                .arg(format!("{}", instructions.output.clone()))
                .arg("-x")
                .arg("assembler")
                .arg("-nostdlib")
                .arg("-static")
                .arg(format!("{}", asm_output))
                .output()
                .expect("Failed to execute assembler!");
            if assembler_output.status.success() {
                println!("Successfully compiled to file \"{}\"", instructions.output.clone());
            } else {
                println!("Assembler failed with error: {}", String::from_utf8_lossy(&assembler_output.stderr));
                exit(11);
            }
            std::fs::remove_file(asm_output).expect("Failed to remove temporary asm file");
        }
    }
}