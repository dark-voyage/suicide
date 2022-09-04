pub mod download;
pub mod grace;

use std::fs::File;
use std::path::PathBuf;

pub fn get_location() -> PathBuf {
    // Find out the path of home folder
    let home = dirs::home_dir().unwrap();

    // Join the suicide folder
    home.join(".suicide")
}

pub fn suicide_exists() -> bool {
    // So, does the ~/.suicide exist?
    get_location().exists()
}

pub async fn extract(file: File, name: String) {
    let mut zip = zip::ZipArchive::new(file).unwrap();

    zip
        .extract(name)
        .expect("Couldn't extract...");

    log::info!("Assets has been updated successfully!")
}
