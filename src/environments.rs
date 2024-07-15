use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Rows},
    widgets::{Block, Borders, Clear, Row, Table, TableState},
    Frame,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::keys::keys::{Event, Key, Modifier::*};

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
    pub temp_env: Environment,
    pub changed: bool,
    pub selected: usize,
    pub selected_kv: usize,
    state: TableState,
}

impl Into<TempEnv> for Environment {
    fn into(self) -> TempEnv {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_env: self,
            changed: false,
            selected: 0,
            selected_kv: 0,
            state: TableState::default(),
        }
    }
}

impl TempEnv {
    pub fn new() -> Self {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_env: Environment::default(),
            changed: false,
            selected: 0,
            selected_kv: 0,
            state: TableState::default(),
        }
    }
    pub fn update(&mut self, event: &Event) {
        if let Some(modifier) = event.modifier {
            match modifier {
                Control => match event.key {
                    Key::Char('n') => self.create_new_kv(),
                    _ => (),
                },
            }
        }
    }
    pub fn change_environment_subsection(&mut self) {
        match self.sections {
            EnvironmentSubSection::Name => self.sections = EnvironmentSubSection::KVs,
            EnvironmentSubSection::KVs => self.sections = EnvironmentSubSection::Name,
        }
    }
    //pub fn remove_name(&mut self) {
    //    self.temp_envs.remove(self.selected);
    //    self.selected = 0;
    //    self.changed = true;
    //}
    //pub fn remove_kv(&mut self) {
    //    let name = self.temp_envs[self.selected].envs_to_show[self.selected_kv][0].clone();
    //    self.temp_envs[self.selected].envs.remove(&name);
    //    self.temp_envs[self.selected]
    //        .envs_to_show
    //        .remove(self.selected_kv);
    //    self.selected_kv = 0;
    //    self.changed = true;
    //}
    //pub fn clear_temp_envs(&mut self) -> Result<(), Error> {
    //    if let Some(idx) = &mut self.current_env_idx {
    //        let mut found: bool = false;
    //        let name = &self.all_envs[*idx].name;
    //        if let Some(temp) = &self.temp_envs {
    //            for item in temp.temp_envs.iter().enumerate() {
    //                if item.1.name == *name {
    //                    self.current_env_idx = Some(item.0);
    //                    found = true;
    //                    break;
    //                }
    //            }
    //        }
    //        if !found {
    //            self.current_env_idx = None;
    //        }
    //    }
    //    if let Some(te) = &self.temp_envs {
    //        if te.changed {
    //            self.all_envs = te.temp_envs.clone();
    //        }
    //    }
    //    self.temp_envs = None;

    //    let mut env_file = fs::File::create(format!("{}/{}", DATA_DIRECTORY, ENV_PATH))
    //        .map_err(|e| Error::FileOperationsErr(e))?;

    //    if self.all_envs.len() > 0 {
    //        env_file
    //            .write_all(
    //                serde_json::to_string(&self.all_envs.clone())
    //                    .map_err(|e| Error::JsonErr(e))?
    //                    .as_bytes(),
    //            )
    //            .map_err(|e| Error::FileOperationsErr(e))?;
    //    }
    //    Ok(())
    //}
    //pub fn new_environment(&mut self, name: String) {
    //    if let Some(te) = &mut self.temp_envs {
    //        te.temp_envs.push(environments::Environment::new(name));
    //        te.with_name_insertion = false;
    //        te.changed = true;
    //    }
    //}
    //pub fn change_active_env_panel(&mut self) {
    //    if let Some(temp) = &mut self.temp_envs {
    //        temp.change_environment_subsection()
    //    }
    //}
    //pub fn has_new_env_name(&self) -> bool {
    //    match &self.temp_envs {
    //        Some(t) => t.with_name_insertion,
    //        None => false,
    //    }
    //}
    //pub fn has_new_env_kv(&self) -> bool {
    //    match &self.temp_envs {
    //        Some(t) => t.with_kv_insertion,
    //        None => false,
    //    }
    //}
    //pub fn change_active_env_kv(&mut self) {
    //    match &mut self.temp_envs {
    //        Some(t) => t.kv_insertion.is_key_active = !t.kv_insertion.is_key_active,
    //        None => (),
    //    }
    //}
    //pub fn add_to_env_kv(&mut self, name: String, key: String, value: String) {
    //    if name == "".to_string() || key == "".to_string() || value == "".to_string() {
    //        return;
    //    }
    //    for item in self.temp_envs {
    //        if item.name == name {
    //            item.envs.insert(key.clone(), value.clone());
    //            item.envs_to_show.push([key, value]);
    //            break;
    //        }
    //    }
    //    t.kv_insertion.key = "".to_string();
    //    t.kv_insertion.value = "".to_string();
    //    t.with_kv_insertion = false;
    //    t.kv_insertion.is_key_active = true;
    //    t.changed = true;
    //}
    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(rect);
        let title = format!("Environment: {}", self.temp_env.name);
        let block = Block::default().title(title).borders(Borders::ALL);
        f.render_widget(block, rect);
        let items = Table::new(
            self.temp_env
                .envs_to_show
                .iter()
                .map(|item| Row::new(vec![item[0].clone(), item[1].clone()])),
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(Row::new(vec!["Key", "Value"]))
        .block(Block::default().title("Environments").borders(Borders::ALL));
        f.render_widget(Clear, chunks[1]);
        f.render_stateful_widget(items, chunks[1], &mut self.state);
    }
}
