use crate::{err::ErrorHandler, structure::{AstNodeType, AstNodeValue, Scope}};
use std::process::exit;

/// Ooh, my first documentation - I'm excited!
/// Code generation module for my compiler.
/// Sorry, ill make this more official looking later.
/// K so - the Sabbah runtime is stored at ~/.sabbah/runtime.s (sorry)
/// The idea is that Sabbah functions are translated to Assembly labels.
/// And because a language with a _start entrypoint is a bit weird,
/// I made it so the entrypoint is called `main`.
/// And the Sabbah runtime calls it in _start.
/// (That's why you can't have a Sabbah function starting with an underscore.)
/// Andddddddddddddddddddddddddd this is my codegen struct.
/// Have fun, because this is shit!
///
///

pub struct CodeGenerator<'a> {
    input: Scope<'a>,
    error_handler: &'a ErrorHandler,
    indent: u16,
}

pub struct Section {
    entries: Vec<String>
}

pub struct Generated {
    data: Section,
    rodata: Section,
    bss: Section,
    text: Section,
}

impl Generated {
    pub fn new() -> Self {
        Self {
            data:   Section { entries: Vec::new() },
            rodata: Section { entries: Vec::new() },
            bss:    Section { entries: Vec::new() },
            text:   Section { entries: Vec::new() },
        }
    }
    pub fn to_string(&self) -> String {
        let mut generated: String = String::new();
        if !self.data.entries.is_empty() {
            generated.push_str(".section .data\n");
            for entry in &self.data.entries {
                generated.push_str(entry);
            }
        }
        if !self.rodata.entries.is_empty() {
            generated.push_str(".section .rodata\n");
            for entry in &self.rodata.entries {
                generated.push_str(entry);
            }
        }
        if !self.bss.entries.is_empty() {
            generated.push_str(".section .bss\n");
            for entry in &self.bss.entries {
                generated.push_str(entry);
            }
        }
        if !self.text.entries.is_empty() {
            generated.push_str(".section .text\n");
            for entry in &self.text.entries {
                generated.push_str(entry);
            }
        }
        generated
    }
}

impl<'a> CodeGenerator<'a> {
    pub fn new(input: Scope<'a>, error_handler: &'a ErrorHandler) -> Self {
        Self {
            input,
            error_handler,
            indent: 0
        }
    }
    pub fn out(&mut self) -> String {
        let mut generated = Generated::new();

        let i: usize = 0;
        while let Some(statement) = self.input.children.get(i) {
            let j: usize = 0;
            while let Some(node) = statement.children.get(j) {
                match node.node_type {
                    AstNodeType::FunctionIdent => {
                        // get full function
                        j += 1;
                        let function_name: String;
                        if let Some(AstNodeType::FunctionIdent) = statement.children.get(j) {
                            j += 1;
                            if let Some(AstNodeValue::FunctionIdent(name)) = statement.children.get(j) {
                                function_name = name;
                            } else {
                                self.error_handler.comperr(
                                    node.line,
                                    node.column,
                                    String::from("expected function identifier node to have value"),
                                    String::from("Please report this error to GitHub: https://github.com/AmosBarSinai/sabbahc/issues")
                                );
                            }
                        }
                    }
                }
            }
        }

        generated.to_string()
    }
}