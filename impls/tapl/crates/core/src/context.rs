use crate::syntax::{Shift, Substitute};

pub trait Binding: PartialEq + Clone + Shift + Substitute {}

#[derive(Debug, PartialEq, Clone)]
pub struct ContextMember<T: Binding> {
    pub name: String,
    pub binding: T,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Context<T: Binding>(Vec<ContextMember<T>>);

impl<T: Binding> Default for Context<T> {
    fn default() -> Context<T> {
        Context(vec![])
    }
}

#[derive(Debug)]
pub enum ContextError {
    RangeError(String),
}

impl<T: Binding> Context<T> {
    pub fn append_binding(&mut self, el: &ContextMember<T>) {
        self.0.insert(0, el.clone())
    }

    pub fn update_binding(&mut self, name: &str, binding: T) -> Result<(), ContextError> {
        let idx = self.get_idx(name)?;
        self.0[idx] = ContextMember {
            name: name.into(),
            binding: binding.clone(),
        };

        Ok(())
    }

    pub fn with_new_binding(&self, el: &ContextMember<T>) -> Self {
        let mut next = self.clone();

        next.append_binding(el);

        next
    }

    pub fn is_name_bound(&self, find_name: &str) -> bool {
        self.0
            .iter()
            .any(|ContextMember { name, .. }| name == find_name)
    }

    pub fn get_free_name(&self, existing_name: &str) -> String {
        match self.is_name_bound(existing_name) {
            true => self.get_free_name(&format!("{}'", existing_name)),
            false => existing_name.into()
        }
    }

    pub fn get_binding(&self, idx: usize) -> Option<&T> {
        self.0.get(idx).map(|x| &x.binding)
    }

    pub fn get_name(&self, idx: usize) -> Result<String, ContextError> {
        self.0.get(idx).map_or(
            Err(ContextError::RangeError(format!(
                "index {} does not exist in context.",
                idx
            ))),
            |x| Ok(x.name.clone()),
        )
    }

    pub fn get_idx(&self, find_name: &str) -> Result<usize, ContextError> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(idx, ContextMember { name, .. })| match name {
                _ if name == find_name => Some(idx),
                _ => None,
            })
            .ok_or(ContextError::RangeError(format!(
                "{} not found in context",
                find_name
            )))
    }

    pub fn find<
        F: Copy + Fn(&(usize, &ContextMember<T>)) -> bool
    >(&self, find_fn: F) -> Option<(usize, &ContextMember<T>)> {
        self.0
        .iter()
        .enumerate()
        .find(find_fn)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn shift_binding(&self, idx: usize) -> Option<T> {
        let next_idx = (idx + 1) as i32;
        match self.0.get(idx) {
            Some(ctx_member) => Some(ctx_member.binding.shift(next_idx)),
            _ => None
        }
    }
}

pub struct ContextIterator<'a, T: Binding> {
    context: &'a Context<T>,
    index: usize
}

impl<'a, T: Binding> IntoIterator for &'a Context<T> {
    type Item = &'a ContextMember<T>;
    type IntoIter = ContextIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ContextIterator {
            context: self,
            index: 0
        }
    }
}

impl<'a, T:Binding> Iterator for ContextIterator<'a, T> {
    type Item = &'a ContextMember<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.context.0.get(self.index);

        self.index += 1;

        item
    }
}
