use crate::context::*;
use crate::parser::parse;
use crate::syntax::*;
use std::fs;

pub struct Evaluation {
    ctx: Context,
}

impl Evaluation {
    pub fn eval(file_name: &str) -> Result<(), &'static str> {
        let mut context = Context::default();
        let file =
            fs::read_to_string(file_name).expect(&format!("Couldn't read file {}", file_name));

        // let commands = parser::Command
        let commands = parse(&file);

        // let commands = parser::TermParser::parse(&mut context, )
        unimplemented!()
    }
}
