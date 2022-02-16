use crate::context::*;
use crate::syntax::*;

pub enum Command {
    Import(String),
    Eval(FileInfo, Term),
    Bind(FileInfo, Binding)
}

pub struct Evaluation {
    ctx: Context,
    commands: Vec<Command>
}