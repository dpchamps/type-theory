use crate::syntax::*;

#[derive(Debug, PartialEq)]
pub struct ContextMember {
    name: String,
    binding: Binding,
}

#[derive(Debug, PartialEq)]
pub struct Context(Vec<ContextMember>);

impl Default for Context {
    fn default() -> Context {
        Context(vec![])
    }
}

impl Context {
    pub fn add_binding(&mut self, el: ContextMember) {
        self.0.push(el)
    }

    pub fn add_name(&mut self, name: &str) {
        self.add_binding(ContextMember {
            name: name.into(),
            binding: Binding::NameBind,
        })
    }

    pub fn is_name_bound(&self, find_name: &str) -> bool {
        self.0
            .iter()
            .any(|ContextMember { name, .. }| name == find_name)
    }

    pub fn get_free_name(&self, existing_name: &str) -> String {
        match self.is_name_bound(existing_name) {
            true => self.get_free_name(&format!("{}'", existing_name)),
            false => existing_name.into(),
        }
    }

    pub fn lookup_name_by_idx(&self, idx: usize) -> Result<String, String> {
        self.0.get(idx).map_or(
            Err(format!("index {} does not exist in context", idx)),
            |x| Ok(x.name.clone()),
        )
    }

    pub fn lookup_idx_by_name(&self, name_to_find: &str) -> Result<usize, String> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(idx, ContextMember { name, .. })| {
                if name == name_to_find {
                    return Some(idx);
                }

                None
            })
            .ok_or(format!("{} not found in context", name_to_find))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::context::*;

    #[test]
    fn context_add_binding() {
        let mut context = Context::default();

        context.add_binding(ContextMember {
            name: "test".into(),
            binding: Binding::NameBind,
        });

        assert_eq!(
            context.0.contains(&ContextMember {
                name: "test".into(),
                binding: Binding::NameBind
            }),
            true
        );
    }

    #[test]
    fn context_add_named_binding() {
        let mut context = Context::default();

        context.add_name("test");

        assert_eq!(
            context.0.contains(&ContextMember {
                name: "test".into(),
                binding: Binding::NameBind
            }),
            true
        );
    }

    #[test]
    fn context_is_name_bound() {
        let mut context = Context::default();

        context.add_name("exists");

        assert_eq!(context.is_name_bound("exists"), true);
        assert_eq!(context.is_name_bound("nonexists"), false);
    }

    #[test]
    fn context_get_free_name() {
        let mut context = Context::default();

        assert_eq!(context.get_free_name("test"), "test");

        context.add_name("test");

        assert_eq!(context.get_free_name("test"), "test'");
    }

    #[test]
    fn context_lookup_idx_by_name() {
        let mut context = Context::default();

        context.add_name("index_0");
        context.add_name("index_1");

        assert_eq!(context.lookup_idx_by_name("index_0"), Ok(0));

        assert_eq!(context.lookup_idx_by_name("index_1"), Ok(1));

        assert_eq!(
            context.lookup_idx_by_name("doesnt_exist"),
            Err("doesnt_exist not found in context".into())
        );
    }

    #[test]
    fn context_lookup_name_by_idx() {
        let mut context = Context::default();

        context.add_name("index_0");
        context.add_name("index_1");

        assert_eq!(context.lookup_name_by_idx(0), Ok("index_0".into()));

        assert_eq!(context.lookup_name_by_idx(1), Ok("index_1".into()));

        assert_eq!(
            context.lookup_name_by_idx(2),
            Err("index 2 does not exist in context".into())
        )
    }
}
