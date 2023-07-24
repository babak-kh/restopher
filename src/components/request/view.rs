use crate::utils::kv::KV;

enum IndexOpration {
    Increase(usize),
    Decrease(usize),
}

#[derive(Debug)]
pub struct ReqView {
    new_param: Option<KV>,
    new_header: Option<KV>,
    new_name: String,
    select_param_idx: usize,
    select_header_idx: usize,
}

impl ReqView {
    pub fn new() -> Self {
        ReqView {
            new_param: None,
            new_header: None,
            new_name: "".to_string(),
            select_param_idx: 0,
            select_header_idx: 0,
        }
    }
    pub fn has_new_header(&self) -> bool {
        match self.new_header {
            Some(_) => true,
            None => false,
        }
    }
    pub fn has_new_param(&self) -> bool {
        match self.new_param {
            Some(_) => true,
            None => false,
        }
    }
    pub fn add_to_active_header(&mut self, ch: char) {
        if let Some(h) = &mut self.new_header {
            h.add_to_active(ch);
        };
    }
    pub fn add_to_active_param(&mut self, ch: char) {
        if let Some(h) = &mut self.new_param {
            h.add_to_active(ch);
        };
    }
    pub fn remove_from_active_param(&mut self) {
        if let Some(h) = &mut self.new_param {
            h.remove_from_active();
        };
    }
    pub fn remove_from_active_header(&mut self) {
        if let Some(h) = &mut self.new_param {
            h.remove_from_active();
        };
    }
    pub fn change_active_header(&mut self) {
        if let Some(h) = &mut self.new_header {
            h.change_active()
        }
    }
    pub fn change_active_param(&mut self) {
        if let Some(h) = &mut self.new_param {
            h.change_active()
        }
    }
    pub fn is_key_active_in_header(&self) -> bool {
        if let Some(h) = &self.new_header {
            return h.is_key_active();
        }
        false
    }
    pub fn is_key_active_in_param(&self) -> bool {
        if let Some(h) = &self.new_param {
            return h.is_key_active();
        }
        false
    }
    pub fn initiate_new_header(&mut self) {
        self.new_header = Some(KV::new());
    }
    pub fn remove_new_header(&mut self) {
        self.new_header = None;
    }
    pub fn initiate_new_param(&mut self) {
        self.new_param = Some(KV::new());
    }
    pub fn remove_new_param(&mut self) {
        self.new_header = None;
    }
    pub fn current_set_header(&self) -> (String, String, bool) {
        if let Some(h) = &self.new_header {
            return (h.get_key(), h.get_value(), true);
        };
        ("".to_string(), "".to_string(), false)
    }
    pub fn current_set_param(&self) -> (String, String, bool) {
        if let Some(h) = &self.new_param {
            return (h.get_key(), h.get_value(), true);
        };
        ("".to_string(), "".to_string(), false)
    }
    pub fn header_idx_ops(&mut self, opt: IndexOpration, len: i32) {
        match opt {
            IndexOpration::Increase(x) => {
                let new_idx = self.select_header_idx + x;
                if new_idx >= len.try_into().unwrap() {
                    self.select_header_idx = 0;
                    return;
                }
                self.select_header_idx = new_idx;
            }
            IndexOpration::Decrease(x) => {
                let current_idx: i32 = self.select_header_idx.try_into().unwrap();
                if current_idx - (x as i32) < 0 {
                    self.select_header_idx = len as usize;
                    return;
                }
                self.select_header_idx -= x;
            }
        }
    }
    pub fn param_idx_ops(&mut self, opt: IndexOpration, len: i32) {
        match opt {
            IndexOpration::Increase(x) => {
                let new_idx = self.select_param_idx + x;
                if new_idx >= len.try_into().unwrap() {
                    self.select_param_idx = 0;
                    return;
                }
                self.select_param_idx = new_idx;
            }
            IndexOpration::Decrease(x) => {
                let current_idx: i32 = self.select_param_idx.try_into().unwrap();
                if current_idx - (x as i32) < 0 {
                    self.select_param_idx = len as usize;
                    return;
                }
                self.select_param_idx -= x;
            }
        }
    }
    pub fn param_idx(&self) -> usize {
        self.select_param_idx
    }
    pub fn header_idx(&self) -> usize {
        self.select_header_idx
    }
}
