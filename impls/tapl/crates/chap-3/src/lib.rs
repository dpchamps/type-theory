#![feature(box_patterns)]

#[derive(Debug, PartialEq)]
struct FileInfo {
    filename: String,
    line: u32,
    col: u32
}

#[derive(Debug, PartialEq)]
enum Value {
    FileInfo(FileInfo),
    UNKNOWN
}

#[derive(Debug, PartialEq)]
enum Term {
    True(Value),
    False(Value),
    If(Value, Box<Term>, Box<Term>, Box<Term>),
    Zero(Value),
    Succ(Value, Box<Term>),
    Pred(Value, Box<Term>),
    IsZero(Value, Box<Term>)
}

fn eval(t: Term) -> Result<Term, String> {
    match t {
        Term::If(_, box Term::True(_), box t2, _) => Ok(t2),
        _ => Err("unimplemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn boxing(){
        let if_term = Term::If(
            Value::UNKNOWN, 
            Box::new(Term::True(Value::UNKNOWN)),
            Box::new(Term::True(Value::UNKNOWN)),
            Box::new(Term::False(Value::UNKNOWN))
        );

        assert_eq!(eval(if_term), Ok(Term::True(Value::UNKNOWN)))
    }
}
