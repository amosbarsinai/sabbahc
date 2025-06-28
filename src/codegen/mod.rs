use crate::{err::ErrorHandler, structure::Scope};

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
    error_handler: &'a ErrorHandler
}

impl<'a> CodeGenerator<'a> {
    pub fn new(input: Scope<'a>, error_handler: &'a ErrorHandler) -> Self {
        Self {
            input,
            error_handler
        }
    }
    pub fn r#gen /* this is the peak of the process! */() {
        
    }
}