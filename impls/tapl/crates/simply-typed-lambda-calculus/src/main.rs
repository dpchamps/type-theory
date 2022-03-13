#![feature(box_patterns)]
#![feature(box_syntax)]

use core::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum SimpleType {
    Unknown,
    Arrow(Box<SimpleType>, Box<SimpleType>),
    Bool,
}

impl fmt::Display for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleType::Arrow(box t1, box t2) => write!(f, "{} → {}", t1, t2),
            SimpleType::Bool => write!(f, "BOOL"),
            SimpleType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleBinding {
    NameBind,
    VarBind(SimpleType),
}

impl fmt::Display for SimpleBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleBinding::NameBind => write!(f, "NameBind"),
            SimpleBinding::VarBind(t) => write!(f, "VarBind({})", t),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Var {
    index: i32,
    container_size: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum SimpleTerm {
    Var(Var),
    Abstraction(String, SimpleType, Box<SimpleTerm>),
    Application(Box<SimpleTerm>, Box<SimpleTerm>),
    True,
    False,
    Conditional(Box<SimpleTerm>, Box<SimpleTerm>, Box<SimpleTerm>),
}

impl Shift for SimpleBinding {}
impl Substitute for SimpleBinding {}
impl Binding for SimpleBinding {}

impl Shift for SimpleTerm {}
impl Substitute for SimpleTerm {}
impl Term for SimpleTerm {}

impl Type for SimpleType {}

#[derive(Debug, PartialEq)]
enum TypeCheckError<B: Binding, T: Term> {
    IncorrectBinding(B),
    BindingNotFound,
    Fail(T, SimpleType, SimpleType),
}

impl TypeCheck<SimpleBinding, SimpleType, TypeCheckError<SimpleBinding, SimpleTerm>> for SimpleTerm {
    fn get_type(
        ctx: &Context<SimpleBinding>,
        idx: i32,
    ) -> Result<SimpleType, TypeCheckError<SimpleBinding, SimpleTerm>> {
        match ctx.get_binding(idx as usize) {
            Some(SimpleBinding::VarBind(ty)) => Ok(ty.clone()),
            Some(incorrect_binding) => {
                Err(TypeCheckError::IncorrectBinding(incorrect_binding.clone()))
            }
            None => Err(TypeCheckError::BindingNotFound),
        }
    }

    fn type_of(
        &self,
        ctx: &Context<SimpleBinding>,
    ) -> Result<SimpleType, TypeCheckError<SimpleBinding, SimpleTerm>> {
        match self {
            SimpleTerm::Var(Var { index, .. }) => SimpleTerm::get_type(ctx, *index),
            SimpleTerm::Abstraction(param, ty_t1, box t2) => {
                let name = ctx.get_free_name(param);
                let next_ctx = ctx.with_new_binding(&ContextMember {
                    name,
                    binding: SimpleBinding::VarBind(ty_t1.clone()),
                });
                let ty_t2 = t2.type_of(&next_ctx)?;

                Ok(SimpleType::Arrow(box ty_t1.clone(), box ty_t2.clone()))
            }
            SimpleTerm::Application(box t1, box t2) => {
                let ty_t1 = t1.type_of(ctx)?;
                let ty_t2 = t2.type_of(ctx)?;

                match ty_t1 {
                    SimpleType::Arrow(box ty_t11, box ty_t22) if ty_t11 == ty_t2 => Ok(ty_t22),
                    SimpleType::Arrow(box ty_11, box ty_t22) => {
                        Err(TypeCheckError::Fail(self.clone(), ty_11, ty_t22))
                    }
                    _ => Err(TypeCheckError::Fail(
                        self.clone(),
                        SimpleType::Arrow(box SimpleType::Unknown, box SimpleType::Unknown),
                        ty_t1,
                    )),
                }
            }
            SimpleTerm::True | SimpleTerm::False => Ok(SimpleType::Bool),
            SimpleTerm::Conditional(box t1, box t2, box t3) => {
                let t_if = t1.type_of(ctx)?;
                if t_if != SimpleType::Bool {
                    return Err(TypeCheckError::Fail(self.clone(), SimpleType::Bool, t_if));
                };

                let t_t2 = t2.type_of(ctx)?;
                let t_t3 = t3.type_of(ctx)?;

                if t_t2 != t_t3 {
                    return Err(TypeCheckError::Fail(self.clone(), t_t2, t_t3));
                }

                Ok(t_t2)
            }
        }
    }
}

fn main() {
    let mut context = Context::<SimpleBinding>::default();
    // let type_checker = Typechecker{ context: &context};
}

#[cfg(test)]
mod test {
    use crate::*;
    use core::*;

    #[test]
    fn typecheck_bool() {
        let context = Context::<SimpleBinding>::default();
        assert_eq!(SimpleTerm::True.type_of(&context), Ok(SimpleType::Bool));
        assert_eq!(SimpleTerm::False.type_of(&context), Ok(SimpleType::Bool));
    }

    #[test]
    fn type_check_var() {
        let mut context = Context::<SimpleBinding>::default();
        let var = SimpleTerm::Var(Var {
            index: 0,
            container_size: 0,
        });

        context.append_binding(&ContextMember {
            name: "test".into(),
            binding: SimpleBinding::VarBind(SimpleType::Bool),
        });

        assert_eq!(var.type_of(&context), Ok(SimpleType::Bool));

        context.append_binding(&ContextMember {
            name: "somethingElse".into(),
            binding: SimpleBinding::NameBind,
        });

        assert_eq!(
            var.type_of(&context),
            Err(TypeCheckError::IncorrectBinding(SimpleBinding::NameBind))
        );

        let var = SimpleTerm::Var(Var {
            index: 10,
            container_size: 0,
        });

        assert_eq!(var.type_of(&context), Err(TypeCheckError::BindingNotFound));
    }

    #[test]
    fn type_check_conditional() {
        let mut context = Context::<SimpleBinding>::default();

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::True,
            box SimpleTerm::False,
            box SimpleTerm::True,
        );

        assert_eq!(conditional.type_of(&context), Ok(SimpleType::Bool));

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::Abstraction("x".into(), SimpleType::Bool, box SimpleTerm::False),
            box SimpleTerm::False,
            box SimpleTerm::True,
        );

        assert_eq!(
            conditional.type_of(&context),
            Err(TypeCheckError::Fail(
                conditional,
                SimpleType::Bool,
                SimpleType::Arrow(box SimpleType::Bool, box SimpleType::Bool)
            ))
        );

        let conditional = SimpleTerm::Conditional(
            box SimpleTerm::True,
            box SimpleTerm::False,
            box SimpleTerm::Abstraction("x".into(), SimpleType::Bool, box SimpleTerm::False),
        );

        assert_eq!(
            conditional.type_of(&context),
            Err(TypeCheckError::Fail(
                conditional.clone(),
                SimpleType::Bool,
                SimpleType::Arrow(box SimpleType::Bool, box SimpleType::Bool)
            ))
        );
    }

    #[test]
    fn abstraction() {
        let mut context = Context::<SimpleBinding>::default();

        context.append_binding(&ContextMember {
            name: "yabadabadoo".into(),
            binding: SimpleBinding::VarBind(SimpleType::Arrow(box SimpleType::Bool, box SimpleType::Unknown)),
        });

        // λx. y
        let abstraction = SimpleTerm::Abstraction(
            "x".into(),
            SimpleType::Bool,
            box SimpleTerm::Var(Var {
                index: 1,
                container_size: 0,
            }),
        );

        assert_eq!(
            abstraction.type_of(&context),
            Ok(SimpleType::Arrow(
                box SimpleType::Bool,
                box SimpleType::Arrow(
                    box SimpleType::Bool,
                    box SimpleType::Unknown
                )
            ))
        )
    }
}
