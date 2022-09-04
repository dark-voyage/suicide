use std::ffi::OsString;
use crate::utils::{get_location, grace, suicide_exists};

pub async fn list() {
    if !suicide_exists() {
        grace::exit("There is no quotes directory at ~/.suicide! Did you initialize suicide?")
    }

    let entries: Vec<OsString> = std::fs::read_dir(get_location()).unwrap().map(|entry| entry.unwrap().file_name()).collect();

    for path in entries {
        println!("{}", path.to_str().unwrap())
    }
}