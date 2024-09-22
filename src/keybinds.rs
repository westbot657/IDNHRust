use std::collections::HashMap;



pub struct Keybinds {
    pub bindings: HashMap<String, String>,
    bind: String
}

impl Keybinds {

    pub fn new() -> Self {

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
                if bind == p {
                    return true
                }
            }
            false
        }
        else {
            false
        }
    }

}


