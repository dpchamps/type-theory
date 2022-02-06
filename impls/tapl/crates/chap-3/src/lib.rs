#![feature(box_patterns)]
#![feature(box_syntax)]

#[derive(Debug, PartialEq)]
enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>)
}

fn eval(t: Term) -> Result<Term, String> {
    match t {
        Term::If(box Term::True, box t2, _) => Ok(t2),
        Term::If(box Term::False, _, box t3) => Ok(t3),
        Term::If(box t1, t2, t3) => {
            let t_prime = eval(t1)?;

            Ok(Term::If(box t_prime, t2, t3))
        },
        _ => Err("unimplemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    fn create_true_if(return_term: Term) -> Term {
        Term::If(
            Box::new(Term::True),
            Box::new(return_term),
            Box::new(Term::False)
        )
    }
    #[test]
    fn if_term_true(){
        let if_term = create_true_if(Term::True);

        assert_eq!(eval(if_term), Ok(Term::True));
    }

    #[test]
    fn if_term_if(){
        let inner_if = Term::If(
            box Term::False,
            box Term::True,
            box Term::False
        );

        let if_if_term = Term::If(
            box inner_if,
            box Term::False,
            box Term::True
        );

        assert_eq!(eval(if_if_term), Ok(Term::If(box Term::False, box Term::False, box Term::True)));
    }
}
