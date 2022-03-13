use crate::context::{Binding, Context};
use crate::syntax::{Term};

pub trait Type: PartialEq + Clone {}

pub trait TypeCheck<B: Binding, T: Type, E> {
    fn get_type(ctx: &Context<B>, idx: i32) -> Result<T, E>
    where 
        Self : Term;
    fn type_of(&self, ctx: &Context<B>) -> Result<T, E>
    where 
        Self : Term;
}
