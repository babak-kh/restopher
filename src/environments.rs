use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    name: &'static str,
    envs: HashMap<String, String>,
}
impl Environment {
    pub fn new(name: &'static str) -> Self {
        Environment {
            name,
            envs: HashMap::new(),
        }
    }
}
