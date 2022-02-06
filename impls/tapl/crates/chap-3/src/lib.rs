#![feature(box_patterns)]
#![feature(box_syntax)]

#[derive(Debug, PartialEq)]
enum EvaluationError {
    Stuck(Box<Term>)
}

#[derive(Debug, PartialEq, Clone)]
enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>)
}

fn is_numeric_value(t: &Term) -> bool {
    match t {
        Term::Zero => true,
        Term::Succ(box t1) => is_numeric_value(t1),
        _ => false
    }
}

fn is_value(t: &Term) -> bool {
    match t {
        Term::True => true,
        Term::False => true,
        _ if is_numeric_value(t) => true,
        _ => false
    }
}

fn eval_step(t: &Term) -> Option<Term> {
    match t {
        Term::If(box Term::True, box t2, _) => Some(t2.clone()),
        Term::If(box Term::False, _, box t3) => Some(t3.clone()),
        Term::If(box t1, t2, t3) => {
            let t_prime = eval_step(t1)?;

            Some(Term::If(box t_prime, t2.clone(), t3.clone()))
        },
        Term::Succ(box t1) => {
            let t_prime = eval_step(t1)?;

            Some(Term::Succ(box t_prime))
        },
        Term::Pred(box Term::Zero) => Some(Term::Zero),
        Term::Pred(box Term::Succ(box nv)) if is_numeric_value(nv) => {
            Some(nv.clone())
        },
        Term::Pred(box t1) => {
            let t_prime = eval_step(t1)?;

            Some(Term::Pred(box t_prime))
        },
        Term::IsZero(box Term::Zero) => Some(Term::True),
        Term::IsZero(box Term::Succ(box nv)) if is_numeric_value(nv) => Some(Term::False),
        Term::IsZero(box t1) => {
            let t_prime = eval_step(t1)?;

            Some(Term::IsZero(box t_prime))
        },
        _ => None
    }
}

fn eval(t: &Term) -> Result<Term, EvaluationError> {
    match eval_step(&t) {
        Some(t_prime) => eval(&t_prime),
        _ =>  {
            let evaluated_term = t.clone();

            if is_value(t) {
                return Ok(evaluated_term);
            }

            Err(EvaluationError::Stuck(box evaluated_term))
        }
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
    fn eval_step_if_term_true(){
        let if_term = create_true_if(Term::True);

        assert_eq!(eval_step(&if_term), Some(Term::True));
    }

    #[test]
    fn eval_step_if_term_if(){
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

        assert_eq!(eval_step(&if_if_term), Some(Term::If(box Term::False, box Term::False, box Term::True)));
    }

    #[test]
    fn eval_true(){
        let t = Term::True;

        assert_eq!(eval(&t), Ok(Term::True));
    }

    #[test]
    fn eval_if_true(){
        let if_true = Term::If(box Term::True, box Term::False, box Term::True);

        assert_eq!(eval(&if_true), Ok(Term::False));
    }

    #[test]
    fn eval_if_false(){
        let if_false = Term::If(box Term::False, box Term::True, box Term::False);

        assert_eq!(eval(&if_false), Ok(Term::False));
    }

    #[test]
    fn eval_nested_if(){
        let root_if = Term::If(box Term::True, box Term::False, box Term::True);
        let n1_if = Term::If(box root_if, box Term::True, box Term::False);
        let n2_if = Term::If(box n1_if, box Term::True, box Term::False);

        assert_eq!(eval(&n2_if), Ok(Term::False));
    }

    #[test]
    fn eval_succ(){
        let succ = Term::Succ(box Term::Zero);

        assert_eq!(eval(&succ), Ok(Term::Succ(box Term::Zero)))
    }

    #[test]
    fn eval_pred_zero(){
        let pred_zero = Term::Pred(box Term::Zero);

        assert_eq!(eval(&pred_zero), Ok(Term::Zero));
    }

    #[test]
    fn eval_pred_succ_nv(){
        let term = Term::Pred(box Term::Succ(box Term::Zero));

        assert_eq!(eval(&term), Ok(Term::Zero));
    }

    #[test]
    fn eval_step_pred(){
        let pred_term = Term::Pred(box Term::Pred(box Term::Zero));

        assert_eq!(eval_step(&pred_term), Some(Term::Pred(box Term::Zero)));
    }
    
    #[test]
    fn eval_is_zero_zero(){
        let is_zero_zero = Term::IsZero(box Term::Zero);

        assert_eq!(eval(&is_zero_zero), Ok(Term::True));
    }

    #[test]
    fn eval_is_zero_succ(){
        let is_zero_succ = Term::IsZero(box Term::Succ(box Term::Zero));

        assert_eq!(eval(&is_zero_succ), Ok(Term::False));
    }

    #[test]
    fn eval_is_zero(){
        let is_zero = Term::IsZero(box Term::Pred(box Term::Succ(box Term::Zero)));
        let is_not_zero = Term::IsZero(box Term::Succ(box Term::Succ(box Term::Zero)));

        assert_eq!(eval(&is_zero), Ok(Term::True));
        assert_eq!(eval(&is_not_zero), Ok(Term::False));
    }

    #[test]
    fn eval_stuck(){
        let stuck_term = Term::Pred(box Term::False);

        assert_eq!(eval(&stuck_term), Err(EvaluationError::Stuck(box stuck_term)))
    }

    #[test]
    fn eval_complex_stuck(){
        let complex_stuck_term = Term::If(
            box Term::False,
            box Term::True,
            box Term::If (
                box Term::True,
                box Term::Succ(
                    box Term::Succ(
                        box Term::True
                    )
                ),
                box Term::Zero
            )
        );

        assert_eq!(eval(&complex_stuck_term), Err(EvaluationError::Stuck(box Term::Succ(box Term::Succ(box Term::True)))))
    }
}
