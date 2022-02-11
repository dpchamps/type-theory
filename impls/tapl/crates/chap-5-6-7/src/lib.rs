#![feature(box_patterns)]
#![feature(box_syntax)]

use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
struct Var {
    index: u32,
    c_size: u32
}


#[derive(PartialEq, Debug)]
enum Binding {
    Namebind
}


#[derive(PartialEq, Debug, Clone)]
enum Term {
    Var(Var),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>)
}

type Context = Vec<(String, Binding)>;

impl Term {
    pub fn is_value(&self, _ctx: &Context) -> bool {
        match self {
            Term::Abs(_, _) => true,
            _ => false
        }
    }

    fn shift_walk(d: i32, cutoff: u32, t: &Term) -> Term {
        match t {
            Term::Var(Var {index, c_size}) => {
                if *index >= cutoff {
                    return Term::Var(Var {index: (*index as i32 + d) as u32, c_size: (*c_size as i32 +d) as u32})
                }

                Term::Var(Var {index: *index, c_size: (*c_size as i32 +d) as u32})
            },
            Term::Abs(name, box term) => {
                Term::Abs(name.to_string(), box Term::shift_walk(d, cutoff+1, &term))
            },
            Term::App(box lhs_term, box rhs_term) => {
                Term::App(
                    box Term::shift_walk(d, cutoff+1, &lhs_term), 
                    box Term::shift_walk (d, cutoff+1, &rhs_term)
                )
            }
        }
    }

    pub fn shift(&self, d: i32) -> Term {
        Term::shift_walk(d, 0, self)
    }

    fn substitute_walk(j: i32, cutoff: i32, s: &Term, t: &Term) -> Term {
        println!("Substitute walk: s: {:?}", s);
        match t {
            Term::Var(Var{index, c_size}) => {
                println!("index: {}, j: {}, cutoff: {}, t: {:?}. Will Substitute: {}", index, j, cutoff, t, *index == (j+(cutoff as i32)) as u32);
                if *index == (j+(cutoff as i32)) as u32 {
                    // perform substition
                    return s.clone().shift(cutoff)
                }

                t.clone()
            },
            Term::Abs(name, box t1) => Term::Abs(name.into(), box Term::substitute_walk(j, cutoff+1, s, &t1)),
            Term::App(box lhs_term, box rhs_term) => 
                Term::App(
                    box Term::substitute_walk(j, cutoff+1, s, &lhs_term),
                    box Term::substitute_walk(j, cutoff+1, s, &rhs_term)
                )
        }

    }

    fn substitute(&self, j: i32, s: &Term) -> Term {
        Term::substitute_walk(j, 0, s, self)
    }

    fn top_level_substitute(&self, s: &Term) -> Term {
        let pre_shift = self.substitute(0, &s.shift(1));
        println!("Pre shift: {:?}", pre_shift);

        pre_shift.shift(-1)
    }

    fn eval_inner(ctx: &Context, t: &Term) -> Option<Term> {
        match t {
            Term::App(box Term::Abs(_, t2), v2) if v2.is_value(ctx) => {
                println!("Substituting {:?} with {:?}", t2, v2);
                Some(t2.top_level_substitute(v2))
            },
            Term::App(v1, box t2) if v1.is_value(ctx) => {
                let t2_prime = Term::eval_inner(ctx, &t2)?;

                Some(Term::App(v1.clone(), box t2_prime))
            },
            Term::App(t1, t2) => {
                let t1_prime = Term::eval_inner(ctx, t1)?;

                Some(Term::App(box t1_prime, t2.clone()))
            }
            _ => None
        }
    }

    fn eval(&self, ctx: &Context) -> Term {
        println!("Evaluating {:?}", self);
        if let Some(t_prime) = Term::eval_inner(ctx, self) {
            println!("t_prime: {:?}", t_prime);
            return t_prime.eval(ctx)
        }

        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn shift() {
        let term = Term::Var(Var{index: 0, c_size: 0});

        assert_eq!(term.shift(2), Term::Var(Var{index: 2, c_size: 2}));

        let term = Term::Abs("x".into(), box Term::Var(Var{index: 0, c_size: 0}));
        assert_eq!(term.shift(2), Term::Abs("x".into(), box Term::Var(Var{index: 0, c_size: 2})));

        let term = Term::Abs("y".into(), box Term::Var(Var{index: 1, c_size: 0}));
        assert_eq!(term.shift(2), Term::Abs("y".into(), box Term::Var(Var{index: 3, c_size: 2})));
    }

    #[test]
    fn substitute() {
        // λx.x => λ.0
        let term = Term::Abs("x".into(), box Term::Var(Var{index: 0, c_size: 1}));

        assert_eq!(
            term.substitute(-1, &Term::Var(Var{index: 7, c_size: 1})), 
            Term::Abs("x".into(), box Term::Var(Var{index: 8, c_size: 2}))
        );

    
        let term = Term::Abs("x".into(), box Term::Var(Var{index: 0, c_size: 1}));

        assert_eq!(
            term.substitute(10, &Term::Var(Var{index: 7, c_size: 1})), 
            term
        );
    }

    #[test]
    fn eval_simple_case() {
        // (λx.x) 0
        let zero = Term::Abs(
            "s".into(),
            box Term::Abs(
                "z".into(),
                box Term::Var(Var{index: 0, c_size: 0})
            )
        );

        let zero_expectation = zero.clone();

        let identity = Term::Abs(
            "x".into(),
            box Term::Var(Var{index: 0, c_size: 0})
        );

        let application = Term::App(box identity, box zero);

        let ctx: Context = vec!();

        assert_eq!(application.eval(&ctx), zero_expectation)
    }

    #[test]
    fn eval_free_variable(){
        // (λx.y) 0
        let ctx: Context = vec!(("x".into(), Binding::Namebind), ("y".into(), Binding::Namebind));

        let open_combinator = Term::Abs(
            "x".into(),
            box Term::Var( Var{index: 2, c_size: 1})
        );

        let zero = Term::Abs(
            "s".into(),
            box Term::Abs(
                "z".into(),
                box Term::Var(Var{index: 0, c_size: 0})
            )
        );

        let application = Term::App(box open_combinator, box zero);

        assert_eq!(
            application.eval(&ctx),
            Term::Var(Var{index: 1, c_size: 0})
        )
    }
}
