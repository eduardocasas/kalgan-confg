use log::{debug, error, info, trace, warn};
use serde::Deserialize;
pub use serde_yaml::Value;
use std::{collections::HashMap, fs, path::Path};

pub(crate) fn generate(source: &str) -> crate::Config {
    info!("");
    info!("***********************************************************************************");
    info!("Crate: Config");
    info!("Start Config Parameters Generation");
    let mut config = crate::Config {
        collection: HashMap::new(),
    };
    if Path::new(source).exists() {
        if Path::new(&source).is_dir() {
            walk_folder_files(Path::new(&source), &mut config.collection);
        } else {
            get_file_content(Path::new(&source), &mut config.collection);
        }
    } else {
        error!("Source path {} not found.", source);
    }
    match config.collection.len() {
        0 => info!("Result: No parameters have been parsed"),
        1 => info!("Result: 1 parameter has been parsed"),
        _ => info!(
            "Result: {} parameters have been parsed",
            config.collection.len()
        ),
    }
    info!("End Config Parameters Generation");
    info!("***********************************************************************************");
    config
}
fn walk_folder_files(dir: &Path, parameters: &mut HashMap<String, Value>) {
    debug!("Reading folder {}...", dir.display());
    match fs::read_dir(dir) {
        Ok(read_dir) => {
            for entry in read_dir {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    debug!("{} is dir", path.display());
                    walk_folder_files(&path, parameters);
                } else {
                    debug!("{} is file", path.display());
                    get_file_content(&path, parameters);
                }
            }
        }
        Err(e) => warn!("{}", e),
    }
}
fn get_file_content(path: &Path, parameters: &mut HashMap<String, Value>) {
    debug!("Reading file {}...", path.display());
    match fs::read_to_string(path) {
        Ok(config_file) => {
            let document = serde_yaml::Deserializer::from_str(&config_file);
            match Value::deserialize(document) {
                Ok(value) => iterate(value, parameters, &mut "".to_string(), ""),
                Err(e) => warn!("{}", e),
            };
        }
        Err(e) => warn!("{}", e),
    };
}
fn iterate(map: Value, parameters: &mut HashMap<String, Value>, subpath: &mut String, prev: &str) {
    for element in map.as_mapping() {
        for (index, value) in element.iter() {
            trace!("Parsing: {:?}", index);
            let key = match index.as_str() {
                Some(p) => p.to_string(),
                None => index.as_i64().unwrap().to_string(),
            };
            if key.contains(".") {
                warn!("Dots are not allowed in key names.");
                warn!("Parameter \"{}\" is skipped.", key);
                break;
            }
            let mut pos = match subpath.rfind(".") {
                Some(num) => num + 1,
                None => 0,
            };
            while !subpath.is_empty() && subpath[pos..].to_string() != prev {
                *subpath = if pos > 0 {
                    subpath[..pos - 1].to_string()
                } else {
                    "".to_string()
                };
                pos = match subpath.rfind(".") {
                    Some(num) => num + 1,
                    None => 0,
                };
            }
            if value.is_mapping() {
                if subpath.is_empty() {
                    subpath.push_str(format!("{}", key.clone()).as_str());
                } else {
                    subpath.push_str(format!(".{}", key.clone()).as_str());
                }
                iterate(value.clone(), parameters, subpath, &key);
            } else {
                let message_name = if subpath.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", subpath.clone(), key)
                };
                parameters.insert(message_name, value.clone());
            }
        }
    }
}
