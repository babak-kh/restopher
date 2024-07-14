use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub envs: HashMap<String, String>,
    pub envs_to_show: Vec<[String; 2]>,
}
impl Environment {
    pub fn new(name: String) -> Self {
        Environment {
            name,
            envs: HashMap::new(),
            envs_to_show: Vec::new(),
        }
    }
    pub fn default() -> Self {
        Environment {
            name: "".to_string(),
            envs: HashMap::new(),
            envs_to_show: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct KV {
    pub is_key_active: bool,
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum EnvironmentSubSection {
    Name,
    KVs,
}
impl KV {
    pub fn new() -> Self {
        KV {
            key: "".to_string(),
            value: "".to_string(),
            is_key_active: true,
        }
    }
}
#[derive(Debug)]
pub struct TempEnv {
    pub sections: EnvironmentSubSection,
    pub temp_envs: Vec<Environment>,
    pub changed: bool,
    pub selected: usize,
    pub selected_kv: usize,
    pub name_insertion: String,
    pub kv_insertion: KV,
    pub with_kv_insertion: bool,
    pub with_name_insertion: bool,
}
impl TempEnv {
    pub fn new(exists: Vec<Environment>) -> Self {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_envs: exists,
            changed: false,
            selected: 0,
            selected_kv: 0,
            with_name_insertion: false,
            with_kv_insertion: false,
            name_insertion: "".to_string(),
            kv_insertion: KV::new(),
        }
    }
    pub fn change_environment_subsection(&mut self) {
        match self.sections {
            EnvironmentSubSection::Name => self.sections = EnvironmentSubSection::KVs,
            EnvironmentSubSection::KVs => self.sections = EnvironmentSubSection::Name,
        }
    }
    pub fn remove_name(&mut self) {
        self.temp_envs.remove(self.selected);
        self.selected = 0;
        self.changed = true;
    }
    pub fn remove_kv(&mut self) {
        let name = self.temp_envs[self.selected].envs_to_show[self.selected_kv][0].clone();
        self.temp_envs[self.selected].envs.remove(&name);
        self.temp_envs[self.selected]
            .envs_to_show
            .remove(self.selected_kv);
        self.selected_kv = 0;
        self.changed = true;
    }
}
