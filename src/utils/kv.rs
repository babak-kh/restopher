use super::text_box::TextBox;

#[derive(Debug)]
struct KVElement {
    text: TextBox,
    active: bool,
}
#[derive(Debug)]
pub struct KV {
    key: KVElement,
    value: KVElement,
}
impl KV {
    pub fn new() -> Self {
        KV {
            key: KVElement {
                text: TextBox::new(),
                active: true,
            },
            value: KVElement {
                text: TextBox::new(),
                active: false,
            },
        }
    }
    pub fn change_active(&mut self) {
        self.value.active = !self.value.active;
        self.key.active = !self.key.active;
    }
    pub fn add_to_active(&mut self, ch: char) {
        if self.key.active {
            self.key.text.push(ch);
            return;
        }
        self.value.text.push(ch)
    }
    pub fn remove_from_active(&mut self) {
        if self.key.active {
            self.key.text.pop();
            return;
        }
        self.value.text.pop();
    }
    pub fn is_key_active(&self) -> bool {
        self.key.active
    }
    pub fn get_key(&self) -> String {
        self.key.text.to_string()
    }
    pub fn get_value(&self) -> String {
        self.value.text.to_string()
    }
}
