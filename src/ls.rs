use std::path::Path;
use setup;
use std::fs;
use regex::Regex;
use node_version::NodeVersion;

fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_)       => false
    }
}

fn directory_name(full_path: &String) -> String {
    let components = Path::new(full_path).components();
    components.last().unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .into()
}

fn is_version_directory(path: &String) -> bool {
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    re.is_match(path)
}

fn follow_symlink() -> Option<String> {
    let home_directory = setup::avm_directory();
    let path = fs::read_link(Path::new(&home_directory).join("bin"));
    if path.is_err() {
        match fs::read_link(Path::new(&home_directory).join("bin").join("node")) {
            Ok(p) => Some(p.as_os_str()
                          .to_str()
                          .unwrap()
                          .into()),
            Err(_) => None
        }
    } else {
        Some(path.unwrap()
             .as_os_str()
             .to_str()
             .unwrap()
             .into())
    }
}

pub fn current_version() -> Option<NodeVersion> {
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let path_str = match follow_symlink() {
        Some(p) => p,
        None => return None
    };
    match re.captures_iter(&path_str).next() {
        Some(m) => {
            match m.at(0) {
                Some(version) => {
                    Some(NodeVersion {
                        name: version.to_string(),
                        path: path_str.replace("/bin", "").to_string()
                    })
                }
                None => None
            }
        },
        None => Some(NodeVersion{
            name: path_str.to_string(),
            path: path_str.to_string()
        })
    }
}

pub fn ls_versions() -> Vec<NodeVersion> {
    if !setup::home_directory_existant() {
        return vec!();
    }
    let home = setup::avm_directory();
    let mut installed_versions = Vec::new();
    for path in fs::read_dir(home).unwrap() {
        let path_str = path.unwrap().path().display().to_string();
        if is_directory(&path_str) && is_version_directory(&path_str) {
            let version = NodeVersion{
                name: directory_name(&path_str),
                path: path_str.to_string()
            };
            installed_versions.push(version);
        }
    }
    installed_versions
}
