use std::collections::HashMap;
use crate::settings::Settings;

pub struct Keybinds {
    pub bindings: HashMap<String, String>,
    bind: String
}

impl Keybinds {

    pub fn new(settings: &Settings) -> Self {

        let bindings_raw = settings.get::<String>("Keybinds").unwrap();
        
        let bindings_toml1: toml::Table = toml::from_str(&("A = ".to_string() + &bindings_raw)).unwrap();
        let bindings_toml = bindings_toml1.get("A").unwrap().as_table().unwrap();
        
        let mut bindings: HashMap<String, String> = HashMap::new();
        
        for (k, v) in bindings_toml {
            if v.is_str() {
                bindings.insert(k.to_string(), v.as_str().unwrap().to_string());
            }
        }
            

        Self {
            bindings,
            bind: "".to_string()
        }
    }

    pub fn push_key(&mut self, key: &str) {
        if self.bind.contains(&(key.to_string() + "+")) {
            return
        }
        self.bind += &(key.to_string() + "+");
    }

    pub fn pop_key(&mut self, key: &str) {
        self.bind = self.bind.replace(&(key.to_string() + "+"), "");
    }

    pub fn matches_any(&self) -> bool {
        // println!("Testing if any keybind matches '{}'", self.bind);
        for (k, v) in &self.bindings {
            if self.check_binding(k) {
                return true
            }
        }
        false
    }

    pub fn check_binding(&self, binding: &str) -> bool {
        if self.bindings.contains_key(binding) {
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


