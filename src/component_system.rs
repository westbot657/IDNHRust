use std::collections::HashMap;
use uuid::Uuid;
use crate::component::Component;
use crate::macros::cast_component;

struct ComponentSystem {
    components: HashMap<String, Box<dyn Component>>
}

impl ComponentSystem {
    fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    fn add_component(&mut self, uuid: impl ToString, component: Box<dyn Component>) {
        self.components.insert(uuid.to_string(), component);
    }

    fn get_new_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }
    
    fn get_component<T>(&mut self, uuid: impl ToString) -> &mut T where T: Component {
        
        cast_component!(self.components.get_mut(&uuid.to_string()).unwrap() => mut T)
    }

    fn remove_component(&mut self, uuid: impl ToString) {
        self.components.remove(&uuid.to_string());
    }
    
}


struct ComponentRef {
    uuid: String,
}

impl ComponentRef {
    fn new(uuid: impl ToString) -> Self {
        Self {
            uuid: uuid.to_string(),
        }
    }
}

