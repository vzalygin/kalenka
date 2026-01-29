use crate::error::CompilerError;

pub struct Context;

impl Context {
    pub fn emit_err(&self, err: CompilerError) {
        println!("ERR {}", err);
    }

    pub fn emit_debug(&self, msg: &str) {
        println!("INFO {}", msg);
    }
}
