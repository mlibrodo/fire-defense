use super::Policy;
use std::collections::HashMap;

#[derive(Default)]
pub struct PolicyRegistry {
    by_name: HashMap<String, Policy>,
}

impl PolicyRegistry {
    pub fn new() -> Self {
        let mut r = Self::default();
        for p in [
            Policy::Observe,
            Policy::Prepare,
            Policy::Defend,
            Policy::Contain,
            Policy::Suppress,
        ] {
            r.by_name.insert(p.to_string(), p);
        }
        r
    }
    pub fn resolve(&self, name: &str) -> Option<Policy> {
        self.by_name.get(name).copied()
    }
}
