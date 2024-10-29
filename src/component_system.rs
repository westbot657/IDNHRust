use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use crate::app::App;
use crate::component::Component;

pub struct ComponentSystem {
    pub components: HashMap<String, Box<dyn Component>>
}

impl ComponentSystem {
    pub fn new() -> Self {
        
        Self {
            components: HashMap::new()
        }
    }
    
    /// Please re-add the taken element with add() unless you are deleting the element
    pub fn take(&mut self, uuid: impl ToString) -> Option<Box<dyn Component>> {
        self.components.remove(&uuid.to_string())
    }
    
    pub fn add(&mut self, uuid: impl ToString, component: Box<dyn Component>) {
        self.components.insert(uuid.to_string(), component);
    }
    
    pub fn wrap(&mut self, comp: Box<dyn Component>) -> CompRef {
        let uuid = Uuid::new_v4().to_string();
        let comp_ref = CompRef::new(&uuid);
        comp_ref.restore(self, comp);
        comp_ref
    }
    
}


pub struct CompRef {
    pub uuid: String,
}

impl CompRef {
    pub fn new(uuid: impl ToString) -> Self {
        Self {
            uuid: uuid.to_string(),
        }
    }
    
    pub fn get(&self, sys: &mut ComponentSystem) -> Option<Box<dyn Component>> {
        sys.take(&self.uuid)
    }
    
    pub fn restore(&self, sys: &mut ComponentSystem, comp: Box<dyn Component>) {
        sys.add(&self.uuid, comp);
    }
    
}

impl Component for CompRef {
    fn update(&mut self, app: &mut App) {
        let comp = self.get(&mut app.component_system);
        if let Some(mut child) = comp {
            child.update(app);
            
            self.restore(&mut app.component_system, child);
        }
    }


    fn destroy(self) {
    }
}

pub trait SystematicComponent {
    fn systemize(self, system: &mut ComponentSystem) -> CompRef;
}

impl<T: 'static + Component> SystematicComponent for T {
    fn systemize(self, system: &mut ComponentSystem) -> CompRef {
        system.wrap(Box::new(self))
    }
}

