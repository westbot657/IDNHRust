use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::ops::Deref;
use uuid::Uuid;
use crate::app::App;
use crate::component::Component;
use crate::macros::cast_component;

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
    
    pub fn wrap<T: 'static + Component>(&mut self, comp: Box<dyn Component>) -> CompRef<T> {
        let uuid = Uuid::new_v4().to_string();
        let comp_ref = CompRef::new(&uuid);
        comp_ref.restore(self, comp);
        comp_ref
    }
    
}


pub struct CompRef<T: 'static + Component> {
    pub uuid: String,
    pub type_id: TypeId,
    _marker: PhantomData<T>
}

impl<T: 'static + Component> CompRef<T> {
    pub fn new(uuid: impl ToString) -> Self {
        Self {
            uuid: uuid.to_string(),
            type_id: TypeId::of::<T>(),
            _marker: PhantomData
        }
    }
    
    pub fn get(&self, sys: &mut ComponentSystem) -> Option<Box<T>> {
        Some(cast_component!(sys.take(&self.uuid)? => owned T))
    }
    
    pub fn restore(&self, sys: &mut ComponentSystem, comp: Box<dyn Component>) {
        sys.add(&self.uuid, comp);
    }
    
}

impl<T: 'static + Component> Component for CompRef<T> {
    fn update(&mut self, app: &mut App) {
        let comp = self.get::<>(&mut app.component_system);
        if let Some(mut child) = comp {
            child.update(app);
            
            self.restore(&mut app.component_system, child);
        }
    }


    fn destroy(self) {
    }
}


pub trait SystematicComponent<T: 'static + Component> {
    fn systemize(self, system: &mut ComponentSystem) -> CompRef<T>;
}

impl<T: 'static + Component> SystematicComponent<T> for T {
    fn systemize(self, system: &mut ComponentSystem) -> CompRef<T> {
        system.wrap(Box::new(self))
    }
}

