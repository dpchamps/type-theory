#![feature(box_patterns)]
#![feature(box_syntax)]

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Var {
    index: u32,
    c_size: u32
}


#[derive(PartialEq, Debug)]
enum Binding {
    Namebind
}


#[derive(PartialEq, Debug)]
enum Term {
    Var(Var),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>)
}

type Context = HashMap<String, Binding>;

impl Term {
    pub fn is_value(&self, _ctx: Context) -> bool {
        match self {
            Term::Abs(_, _) => true,
            _ => false
        }
    }

    fn shift_walk(d: u32, cutoff: u32, t: &Term) -> Term {
        match t {
            Term::Var(Var {index, c_size}) => {
                if *index >= cutoff {
                    return Term::Var(Var {index: index+d, c_size: c_size+d})
                }

                Term::Var(Var {index: *index, c_size: c_size+d})
            },
            Term::Abs(name, box term) => {
                Term::Abs(name.to_string(), box Term::shift_walk(d, cutoff+1, &term))
            },
            Term::App(box lhs_term, box rhs_term) => {
                Term::App(box Term::shift_walk(d, cutoff+1, &lhs_term), box Term::shift_walk (d, cutoff+1, &rhs_term))
            }
        }
    }

    pub fn shift(&self, d: u32) -> Term {
        Term::shift_walk(d, 0, self)
    }

    fn shift_substitute() -> Term {
        unimplemented!()
    }

    pub fn substitute(&self, ) -> Term {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn shift() {
        let term = Term::Var(Var{index: 0, c_size: 0});

        assert_eq!(term.shift(2), term);
    }

}
