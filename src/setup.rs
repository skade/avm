use std::path::Path;
use std::fs;
use std::env;
use std::io::Error;

fn version_path(version: &String) -> String {
    Path::new(&avm_directory()).join(version)
        .as_path()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn avm_directory() -> String {
    let home_directory = env::home_dir().unwrap();
    let avm = home_directory.join(".avm");
    avm.as_path().to_str().unwrap().to_string()
}

pub fn home_directory_existant() -> bool {
    match fs::metadata(avm_directory()) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

pub fn prepare() -> Result<String, Error> {
    if home_directory_existant() {
        return Ok(avm_directory());
    }
    match fs::create_dir(avm_directory().clone()) {
        Ok(_) => Ok(avm_directory()),
        Err(err) => Err(err)
    }
}

pub fn create_version_directory(version: &String) -> Result<String, Error> {
    let path = version_path(&version);
    match fs::create_dir(&path) {
        Ok(_) => Ok(path.clone()),
        Err(err) => Err(err)
    }
}

pub fn has_version(version_str: &String) -> bool {
    let path = version_path(&version_str);
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false
    }
}

pub fn remove_version(version_str: &String) -> Result<(), Error> {
    let path = version_path(&version_str);
    fs::remove_dir_all(path)
}
