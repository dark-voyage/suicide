use crate::utils;

async fn create() {
    match utils::suicide_exists() {
        true => {}
        false => tokio::fs::create_dir(utils::get_location())
            .await
            .expect("Couldn't create the suicide folder at home folder"),
    }
}

pub async fn main() {
    create().await;
}
