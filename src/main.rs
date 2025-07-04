use std::fs::File;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::process::exit;

use crate::err::ErrorHandler;

mod codegen;
mod err;
mod structure;
mod tokenizer;
mod typing;

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
enum EarlyExit {
    Version,
    Help,
}

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
        let mut input: String = String::new();
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
                            _ => {
                                println!("ERROR: Unrecognized mode: {}", args[i + 1]);
                                exit(4);
                            }
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
                    } else {
                        if args[i].starts_with("-") {
                            println!("ERROR: Unrecognized flag/option: {}", args[i]);
                        } else {
                            println!("ERROR: Unexpected argument: {}", args[i]);
                        }
                        exit(2);
                    }
                }
            }
            i += 1;
        }
        if input.is_empty() {
            println!("WARNING: No input file specified, defaulting to stdin");
            input = String::from("/dev/stdin");
        }
        if output.is_empty() {
            // Generate output file name from input
            let input_path = Path::new(&input);
            let mut output_file_name = input_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output")
                .to_string();
            if output_file_name.ends_with(".sbb") {
                output_file_name = output_file_name[0..output_file_name.len() - 4].to_string();
            }
            match mode {
                OutputMode::Assembly => output = format!("{}.s", output_file_name),
                OutputMode::Object => output = format!("{}.o", output_file_name),
                OutputMode::BinaryExecutable => output = format!("{}", output_file_name),
            }
        }
        return CLIInstructions {
            input,
            output,
            mode,
            exit_early,
            force,
        };
    }
}

const VERSION: &str = "0.0.1";

fn find_free_filename(ext: &str) -> String {
    let mut counter = 0;
    loop {
        let filename = format!("tmp{}.{}", counter, ext);
        if !Path::new(&filename).exists() {
            return filename;
        }
        counter += 1;
    }
}

fn main() {
    // Start the timer - how long does it take to compile?
    let start_time = std::time::Instant::now();

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
    let input =
        std::fs::read_to_string(instructions.input.clone()).expect("Failed to read input file");

    let error_handler = ErrorHandler::new(
        input.clone(),
        instructions.input.as_str()
    );
    let mut tokenizer = tokenizer::Tokenizer::new(&input, instructions.input.clone(), &error_handler);
    let tokenized: Vec<tokenizer::Token> = tokenizer.tokenize();
    
    let mut parser = structure::parser::Parser::new(&tokenized, &error_handler);
    let parsed: structure::Scope = parser.parse();
    
    let mut codegener = codegen::CodeGenerator::new(
        parsed,
        &error_handler,
    );
    let generated = codegener.out();

    match instructions.mode {
        OutputMode::Assembly => {
            {
                let mut file = File::create(instructions.output.as_str()).unwrap();
                write!(file, "{}", generated);
            }
        }
        OutputMode::Object => {
            let assembly_filename = find_free_filename("s");
            {
                let mut file = File::create(assembly_filename.clone()).unwrap();
                write!(file, "{}", generated);
            }
            Command::new("as")
                .arg(assembly_filename.clone())
                .arg("-o")
                .arg(instructions.output)
                .output()
                .expect("Assembler error");
            std::fs::remove_file(assembly_filename);
        }
        OutputMode::BinaryExecutable => {
            let assembly_filename = find_free_filename("s");
            let object_filename = find_free_filename("o");
            {
                let mut file = File::create(assembly_filename.clone()).unwrap();
                write!(file, "{}", generated);
            }
            Command::new("as")
                .arg(assembly_filename.clone())
                .arg("-o")
                .arg(object_filename.clone())
                .output()
                .expect("Assembler error");
            Command::new("ld")
                .arg("-o")
                .arg(instructions.output)
                .arg("runtime.o")
                .arg(object_filename.clone())
                .output()
                .expect("Linker error");
            std::fs::remove_file(assembly_filename);
            std::fs::remove_file(object_filename);
        }
    }
    
    let elapsed = start_time.elapsed();
    println!(
        "Compilation completed in {}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}
