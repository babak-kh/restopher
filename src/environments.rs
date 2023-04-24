use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub name: &'a str,
    pub envs: HashMap<String, String>,
}
impl <'a>Environment<'a> {
    pub fn new(name: &'a str) -> Self {
        Environment {
            name,
            envs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentSubSection {
    Name,
    KVs,
}
#[derive(Debug)]
pub struct TempEnv<'a> {
    pub sections: EnvironmentSubSection,
    pub temp_envs: Vec<Environment<'a>>,
    pub changed: bool,
    pub selected: usize,
}
impl <'a>TempEnv<'a> {
    pub fn new(exists: Vec<Environment<'a>>) -> Self {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_envs: exists,
            changed: false,
            selected: 0,
        }
    }
}
