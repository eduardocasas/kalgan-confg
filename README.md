# kalgan-config

Collection of functions to retrieve data and settings parameters defined in yaml files used by Kalgan Framework.

## Examples

This is the yaml file to be used in the following tests:
```yaml
## tests/settings.yaml

user:
  name: John
  is_real: false
  age: 39
  height: 1.78
  children:
    - Huey
    - Dewey
    - Louie
```
```rust
use kalgan_config::Config;

let config: Config = Config::new("tests/settings.yaml");
```
```rust
assert_eq!(config.get("user.name").unwrap(), "John");
```
```rust
assert_eq!(config.get_string("user.name").unwrap(), "John".to_string());
```
```rust
assert_eq!(config.get_bool("user.is_real").unwrap(), false);
```
```rust
assert_eq!(config.get_number("user.age").unwrap(), 39);
```
```rust
assert_eq!(config.get_float("user.height").unwrap(), 1.78);
```
```rust
assert_eq!(config.get_vec("user.children").unwrap(), vec!["Huey", "Dewey", "Louie"]);
```
    
## Documentation

For further information please visit:

* [Official Kalgan Site](https://kalgan.eduardocasas.com)
* [API Documentation on docs.rs](https://docs.rs/crate/kalgan-config/latest)


## License

This crate is licensed under either of the following licenses:

* [MIT License](https://choosealicense.com/licenses/mit/)
* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
