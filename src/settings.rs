use std::{fs, str::FromStr};
use std::any::type_name_of_val;
use toml::{Table, Value};

pub struct Settings {
    default: String,
    config: String,
    default_table: Table,
    table: Table
}


impl Settings {

    pub fn new() -> Self {
        let defaults = fs::read_to_string("assets/default_config.toml").unwrap();

        Self {
            default: defaults.clone(),
            config: defaults.to_string(),
            default_table: defaults.parse::<Table>().unwrap(),
            table: defaults.parse::<Table>().unwrap()
        }
    }


    pub fn save(&mut self) {
        let out_res = toml::to_string_pretty(&self.table);

        if let Ok(out) = out_res {
            if !fs::exists("./data/settings.toml").unwrap() {
                let _ = fs::create_dir_all("./data/");
            }

            fs::write("data/settings.toml", out).unwrap();

        }
    }

    pub fn reload(&mut self) {
        self.load()
    }

    pub fn load(&mut self) {
        let config_res = fs::read_to_string("data/settings.toml");

        if config_res.is_ok() {
            let config = config_res.unwrap();

            let table_res = config.parse::<Table>();

            if table_res.is_ok() {
                let table = table_res.unwrap();
            }

        }

    }

    /// setting_value uses '/' to separate nested paths
    pub fn get<T>(&self, setting_value: &str) -> Result<T, String>
    where
        T: FromStr,
        T::Err: std::fmt::Display, <T as FromStr>::Err: std::fmt::Debug
    {
        let setting = Vec::from_iter(setting_value.split("/"));
        // Attempt to traverse the TOML table and retrieve the value
        let result = self.traverse_table(&setting)
            .and_then(|value| {
                let res = value.to_string().parse::<T>();
                
                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    Err("Value is not of requested type".to_string())
                }
            });

        // If traversal or parsing fails, return the default value
        result.or_else(|_| self.get_default(setting))
    }

    fn traverse_table<'a>(&'a self, setting: &[&str]) -> Result<&'a Value, String> {
        let mut current = &self.table;

        for key in setting.iter().take(setting.len() - 1) {
            match current.get(*key) {
                Some(Value::Table(table)) => current = table,
                _ => return Err(format!("Key '{}' not found or is not a table", key)),
            }
        }

        let last_key = setting.last().ok_or("Empty setting path")?;
        current.get(*last_key).ok_or(format!("Key '{}' not found", last_key))
    }

    pub fn get_default<T>(&self, setting: Vec<&str>) -> Result<T, String>
    where
        T: FromStr,
        T::Err: std::fmt::Display, <T as FromStr>::Err: std::fmt::Debug
    {
        let mut current = &self.default_table;

        for key in setting.iter().take(setting.len() - 1) {
            match current.get(*key) {
                Some(Value::Table(table)) => current = table,
                _ => return Err(format!("Key '{}' not found or is not a table", key)),
            }
        }

        let last_key = setting.last().ok_or("Empty setting path")?;
        match current.get(*last_key) {
            Some(value) => {
                let res = value.to_string().parse::<T>();

                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    Err("Value is not of requested type".to_string())
                }
            }
            None => Err(format!("Key '{}' not found", last_key)),
        }

    }

    pub fn set<T>(&mut self, setting_value: &str, value: T)
    where
        T: Into<Value>,
    {
        let setting = Vec::from_iter(setting_value.split("/"));
        // Convert the input value into a toml::Value
        let value = value.into();

        // Ensure there is at least one key in the path
        if setting.is_empty() {
            eprintln!("Error: setting path cannot be empty.");
            return;
        }

        // Use a helper function to traverse or create intermediate tables
        let mut current = &mut self.table;
        for key in &setting[..setting.len() - 1] {
            // Traverse or create intermediate tables as needed
            current = current.entry(*key).or_insert_with(|| Value::Table(Table::new()))
                .as_table_mut()
                .expect("Failed to insert table");
        }

        // Insert the value at the last key
        if let Some(last_key) = setting.last() {
            current.insert(last_key.to_string(), value);
        } else {
            eprintln!("Error: invalid setting path.");
        }
    }
    
    /// this is meant for things like the editor tutorial which you may not actually want to reset when resetting other stuff
    pub fn set_default<T>(&mut self, setting: Vec<&str>, value: T)
    where
        T: Into<Value>,
    {
        // Convert the input value into a toml::Value
        let value = value.into();

        // Ensure there is at least one key in the path
        if setting.is_empty() {
            eprintln!("Error: setting path cannot be empty.");
            return;
        }

        // Use a helper function to traverse or create intermediate tables
        let mut current = &mut self.default_table;
        for key in &setting[..setting.len() - 1] {
            // Traverse or create intermediate tables as needed
            current = current.entry(*key).or_insert_with(|| Value::Table(Table::new()))
                .as_table_mut()
                .expect("Failed to insert table");
        }

        // Insert the value at the last key
        if let Some(last_key) = setting.last() {
            current.insert(last_key.to_string(), value);
        } else {
            eprintln!("Error: invalid setting path.");
        }
    }


}


