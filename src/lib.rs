//! Collection of functions to retrieve data and settings parameters defined in yaml files.

pub use serde_yaml::Value;
use std::collections::HashMap;
mod parameters;

#[derive(Debug)]
/// The object that keeps the data collection.
///
/// This is the yaml file to be used in the following tests:
/// ```yaml
/// ## tests/settings.yaml
///
/// user:
///   name: John
///   is_real: false
///   age: 39
///   height: 1.78
///   children:
///     - Huey
///     - Dewey
///     - Louie
/// ```
pub struct Config {
    pub collection: HashMap<String, Value>,
}
impl Config {
    /// Creates and return the `Config` instance given the data source path (can be a file or a folder).
    /// # Examples:
    /// ```
    /// use kalgan_config::Config;
    ///
    /// let config: Config = Config::new("tests/settings.yaml");
    /// ```
    pub fn new(source: &str) -> Config {
        parameters::generate(source)
    }
    /// Returns the `serde_yaml::Value` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get("user.name").unwrap(), "John");
    /// ```
    pub fn get(&self, key: &str) -> Result<Value, String> {
        if self.exists(&key) {
            Ok(self.collection[key].clone())
        } else {
            Err(format!("Key \"{}\" not found.", &key))
        }
    }
    /// Returns the value as a `String` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get_string("user.name").unwrap(), "John".to_string());
    /// ```
    pub fn get_string(&self, key: &str) -> Result<String, String> {
        match self.get(&key)?.as_str() {
            Some(string) => Ok(string.to_string()),
            None => Err(format!("Value \"{}\" is not a string.", &key)),
        }
    }
    /// Returns the value as a `bool` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get_bool("user.is_real").unwrap(), false);
    /// ```
    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        match self.get(&key)?.as_bool() {
            Some(bool) => Ok(bool),
            None => Err(format!("Value \"{}\" is not boolean.", &key)),
        }
    }
    /// Returns the value as a `i64` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get_number("user.age").unwrap(), 39);
    /// ```
    pub fn get_number(&self, key: &str) -> Result<i64, String> {
        match self.get(&key)?.as_i64() {
            Some(num) => Ok(num),
            None => Err(format!("Value \"{}\" is not i64/u64.", &key)),
        }
    }
    /// Returns the value as a `f64` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get_float("user.height").unwrap(), 1.78);
    /// ```
    pub fn get_float(&self, key: &str) -> Result<f64, String> {
        match self.get(&key)?.as_f64() {
            Some(num) => Ok(num),
            None => Err(format!("Value \"{}\" is not f64.", &key)),
        }
    }
    /// Returns the value as a `Vec<serde_yaml::Value>` for the given parameter.
    /// # Examples:
    /// ```
    /// # use kalgan_config::Config;
    /// # let config: Config = Config::new("tests/settings.yaml");
    /// assert_eq!(config.get_vec("user.children").unwrap(), vec!["Huey", "Dewey", "Louie"]);
    /// ```
    pub fn get_vec(&self, key: &str) -> Result<Vec<Value>, String> {
        match self.get(&key)?.as_sequence() {
            Some(sequence) => Ok(sequence.clone()),
            None => Err(format!("Value \"{}\" is not a sequence.", &key)),
        }
    }
    pub fn exists(&self, key: &str) -> bool {
        self.collection.contains_key(&key.to_string())
    }
}
