use crate::context::*;
use crate::syntax::*;
use std::fmt;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Term::String(_, s) => write!(f, "{}", s),
            Term::Var(_, Var { name, .. }) => write!(f, "{}", name),
            Term::True(_) => write!(f, "true"),
            Term::False(_) => write!(f, "false"),
            Term::If(_, box t1, box t2, box t3) => {
                write!(f, "if {} then {} else {}", t1, t2, t3)
            }
            Term::Let(_, name, box t1, box t2) => {
                write!(f, "let {} = {} in \n{}", name, t1, t2)
            }
            Term::Projection(_, box t1, n) => {
                write!(f, "{}.{}", t1, n)
            }
            Term::Record(_, fields) => {
                let csv = fields
                    .iter()
                    .map(|(name, term)| format!("{} = {}", name, term))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{ {} }}", csv)
            }
            Term::Abstraction(_, name, box t2) => write!(f, "λ{}. {}", name, t2),
            Term::Application(_, t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Zero(_) | Term::Successor(_, _) | Term::Predecessor(_, _) => write!(
                f,
                "{}",
                self.into_int()
                    .map_or(String::from("NaN"), |x| x.to_string())
            ),
            Term::IsZero(_, t1) => write!(f, "iszero {}", t1),
            Term::Float(_, flt) => write!(f, "{}", flt),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Binding::NameBind => write!(f, ""),
            Binding::TermBind(t) => write!(f, "{}", t),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Command::Bind(_, name, binding) => write!(f, "const {} = {};", name, binding),
            Command::Eval(_, term) => write!(f, "{};", term),
            Command::Import(import) => write!(f, "use {};", import),
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context [\n")?;
        for ContextMember { name, binding } in self.into_iter() {
            write!(f, "\t{} = {}\n", name, binding)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::printer::*;
    use crate::syntax::*;

    #[test]
    fn test_print_simple() {
        let id = "λx. x;";
        let (parsed, _) = parse(id).expect("parse error");

        assert_eq!(format!("{}", parsed[0]), id);
    }

    #[test]
    fn test_print_complex() {
        let input = r#"
        let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) in
        let g = λx. x in
        Y g;
        "#;

        let expectation = r#"let Y = λf. (λx. (f λy. ((x x) y)) λx. (f λy. ((x x) y))) in 
let g = λx. x in 
(Y g);"#;

        let (parsed, _) = parse(input).expect("parse error");

        assert_eq!(format!("{}", parsed[0]), expectation);
    }
}
