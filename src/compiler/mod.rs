use std::panic;

use crate::parser::{Ast, AstNodeType};

fn syscall(name: &str) -> i8 {
    match name {
        "exit" => 60,
        "read" => 0,
        "write" => 1,
        _ => panic!("Unknown syscall name: {}", name),
    }
}

pub struct Assembly {
    data: String,
    text: String,
    bss: String,
    rodata: String,
    entry: String
}
impl Assembly {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            text: String::from("        global _start"),
            bss: String::new(),
            rodata: String::new(),
            entry: String::new()
        }
    }
    pub fn to_string(&self) -> String {
        format!(
"section .data
{}
section .bss
{}
section .rodata
{}
section .text
{}
_start:
{}",
        self.data,
        self.bss,
        self.rodata,
        self.text,
        self.entry
        )
    }
}

pub struct Compiler {
    input: Vec<Ast>,
    indent: usize,
    asm: Assembly,
}

impl Compiler {
    pub fn new(input: Vec<Ast>) -> Self {
        Self {
            input,
            indent: 0,
            asm: Assembly::new(),
        }
    }
    pub fn ln(&mut self, line: &str) {
        self.asm.entry.push_str(&format!("{}{}\n", " ".repeat(self.indent + 8), line));
    }
    pub fn compile(&mut self, timestamp: bool) -> String {
        for i in 0..self.input.len() {
            let ast = &self.input[i];
            let nodes = ast.children.clone();
            for node in nodes {
                if let AstNodeType::ExitStatement(parsed_code) = &node {
                    let code: i32 = parsed_code.unwrap_or(0);
                    self.ln(&format!("mov rax, {}", syscall("exit")));
                    self.ln(&format!("mov rdi, {}", code));
                    self.ln("syscall");
                }
            }
        }
        self.asm.to_string()
    }
}