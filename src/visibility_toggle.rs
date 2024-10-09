
pub struct VisibilityToggle {
    pub visible: bool,
    pub group: String,
}

impl VisibilityToggle {
    pub fn new(group: &str) -> Self {
        
        Self {
            visible: true,
            group: group.to_string(),
        }
    }
    
    pub fn is_spacer(&self) -> bool {
        self.group.is_empty()
    }
}
