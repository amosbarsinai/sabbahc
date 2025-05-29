use std::panic;
use std::collections::HashMap;

use crate::parser::{Ast, AstNodeType};

use crate::parser::ExpressionType;

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
    entrypoint: String,
    calls: HashMap<String, String>,
}
impl Assembly {
    pub fn new() -> Self {
        let mut ret = Self {
            data: String::new(),
            text: String::from("        global _start\n        global exit\n"),
            bss: String::new(),
            rodata: String::new(),
            entrypoint: String::new(),
            calls: HashMap::new(),
        };
        ret.calls.insert("exit".to_string(), format!("        mov rax, {}\n        syscall", syscall("exit")).to_string());
        ret
    }
    pub fn to_string(&self) -> String {
        let mut ret = format!(
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
        self.entrypoint,
        );
        for label in &self.calls {
            ret.push_str(format!("{}:\n", label.0).as_str());
            ret.push_str(format!("{}\n", label.1.as_str()).as_str());
        }
        ret
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
        self.asm.entrypoint.push_str(&format!("{}{}\n", " ".repeat(self.indent + 8), line));
    }
    pub fn compile(&mut self) -> String {
        for i in 0..self.input.len() {
            let children = &self.input[i].children.clone();
            for j in 0..children.len() {
                let node = &children[j];
                match node {
                    AstNodeType::ExitStatement => {
                        if let Some(AstNodeType::Expression(eval_type, expr_type)) = &children.get(j + 1) {
                            if let ExpressionType::IntLiteral(value) = expr_type {
                                self.ln(format!("mov rdi, {}", value).as_str());
                                self.ln("call exit");
                            }
                        }
                    }
                    AstNodeType::Expression(_, _) => {}
                }
            }
        }
        self.asm.to_string()
    }
}