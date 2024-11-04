use serde_json::{Number, Value};
use crate::app::App;
use crate::component::Component;
use crate::component_system::{CompRef, SystematicComponent};

pub struct ShelfTile {
    show_drop_crate: bool,
    tile_reference: CompRef<ObjectTile>
}




pub trait DataAttribute {
    
    fn write(&self, map: &mut Value) -> Result<(), String>;
    
    fn read(&mut self, map: &Value) -> Result<(), String>;
    
}

macro_rules! data_reader {
    ( $map:expr, $key:expr, $branch:pat => $block:expr ) => {
        if $map.is_object() {
            if let Some(table) = $map.as_object() {
                if let Some(val) = table.get($key) {
                    match val {
                        $branch => {
                            $block;
                            return Ok(())
                        },
                        _ => {
                            return Err("Value is of incorrect type".to_string())
                        }
                    }
                } else {
                    return Err("Value is not in map".to_string())
                }
            } else {
                unreachable!();
            }
        } else {
            return Err("`map` is not an Object value".to_string())
        }
    };
}

macro_rules! data_writer {
    ( $map:expr, $key:expr, $value:expr ) => {
        if $map.is_object() {
            if let Some(table) = $map.as_object_mut() {
                table.insert($key, $value);
                Ok(())
            } else {
                unreachable!();
            }
        } else {
            Err("`map` is not an Object value".to_string())
        }
    };
}

pub struct BooleanAttribute {
    pub key: String,
    pub value: bool,
}

impl BooleanAttribute {}
impl DataAttribute for BooleanAttribute {
    fn write(&self, map: &mut Value) -> Result<(), String> {
        data_writer!(map, self.key.to_string(), Value::Bool(self.value))
    }

    fn read(&mut self, map: &Value) -> Result<(), String> {
        data_reader!(map, &self.key, Value::Bool(b) => {self.value = b.clone()})
    }
}

pub struct StringAttribute {
    pub key: String,
    pub value: String,
}

impl StringAttribute {}
impl DataAttribute for StringAttribute {
    fn write(&self, map: &mut Value) -> Result<(), String> {
        data_writer!(map, self.key.to_string(), Value::String(self.value.clone()))
    }

    fn read(&mut self, map: &Value) -> Result<(), String> {
        data_reader!(map, &self.key, Value::String(s) => {self.value = s.clone()})
    }
}


pub struct I64Attribute {
    pub key: String,
    pub value: i64,
}
impl I64Attribute {}
impl DataAttribute for I64Attribute {
    fn write(&self, map: &mut Value) -> Result<(), String> {
        data_writer!(map, self.key.to_string(), Value::Number(Number::from(self.value)))
    }

    fn read(&mut self, map: &Value) -> Result<(), String> {
        data_reader!(map, &self.key, Value::Number(n) => {
            if n.is_i64() {
                self.value = n.as_i64().unwrap();
            } else {
                return Err("Value is not an integer".to_string())
            }
        })
    }
}


pub struct F64Attribute {
    pub key: String,
    pub value: f64,
}
impl F64Attribute {}
impl DataAttribute for F64Attribute {
    fn write(&self, map: &mut Value) -> Result<(), String> {
        data_writer!(map, self.key.to_string(), Value::Number(Number::from_f64(self.value).unwrap()))
    }

    fn read(&mut self, map: &Value) -> Result<(), String> {
        data_reader!(map, &self.key, Value::Number(n) => {
            if n.is_f64() {
                self.value = n.as_f64().unwrap();
            } else {
                return Err("Value is not a number".to_string())
            }
        })
    }
}



pub struct ObjectTile {
    pub position: (i32, i32),
    pub id: String,
    pub name: String,
    pub description: String,
    pub attributes: Vec<Box<dyn DataAttribute>>,
}

impl ObjectTile {
    
    /// Returns a CompRef<ObjectTile> as the ObjectTile is meant to be shared with a ShelfTile
    pub fn new(app: &mut App, id: impl ToString, name: impl ToString, description: impl ToString, attributes: Vec<Box<dyn DataAttribute>>) -> CompRef<ObjectTile> {

        Self {
            position: (0, 0),
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            attributes,
        }.systemize(&mut app.component_system)
    }
}


impl Component for ObjectTile {
    fn update(&mut self, app: &mut App) {
        
    }

}