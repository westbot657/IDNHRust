use std::collections::HashMap;
use crate::settings::Settings;

pub struct Keybinds {
    pub bindings: HashMap<String, String>,
    bind: String
}

impl Keybinds {

    pub fn new(settings: &Settings) -> Self {

        // settings.get("")

        Self {
            bindings: HashMap::new(),
            bind: "".to_string()
        }
    }

    pub fn push_key(&mut self, key: &str) {
        if self.bind.contains(key) {
            return
        }
        self.bind += &(key.to_string() + "+");
    }

    pub fn pop_key(&mut self, key: &str) {
        self.bind = self.bind.replace(&(key.to_string() + "+"), "");
    }

    pub fn matches_any(&self) -> bool {
        println!("{:?}", self.bindings);
        for (k, v) in &self.bindings {
            println!("{}: {}", k, v);
            if self.check_binding(k) {
                return true
            }
        }
        false
    }

    pub fn check_binding(&self, binding: &str) -> bool {
        if self.bindings.contains_key(binding) {
            println!("Contains key");
            let bind = self.bindings.get(binding).unwrap();

            let mut pattern = self.bind.strip_suffix("+").unwrap_or(&self.bind.to_string()).to_string();

            pattern += &(" | ".to_string() + &(self.bind.strip_suffix("+").unwrap_or(&self.bind.to_string())
                .replace("Left Ctrl", "Ctrl")
                .replace("Right Ctrl", "Ctrl")
                .replace("Left Shift", "Shift")
                .replace("Right Shift", "Shift")
                .replace("Left Alt", "Alt")
                .replace("Right Alt", "Alt")
            ));

            for p in pattern.split(" | ") {
                for b in bind.strip_suffix("+").unwrap_or(&bind.to_string()).split(" | ") {
                    if p == b {
                        return true
                    }
                }
            }
            false


        }
        else {
            false
        }
    }

}


