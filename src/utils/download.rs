use std::fs::File;
use reqwest::Client;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn create_client() -> Client {
    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Couldn't build the client")
}

pub async fn download(url: &str, client: Option<&Client>) -> File {
    let mut temporary = tempfile::tempfile().unwrap();

    let bytes = match client {
        Some(client) => client
                .get(url)
                .send()
                .await
                .expect("Problems with Internet connectivity!")
                .bytes()
                .await
                .expect("Can't convert source into bytes!"),

        None => reqwest::get(url)
                .await
                .expect("Problems with Internet connectivity!")
                .bytes()
                .await
                .expect("Can't convert source into bytes!"),
    };

    std::io::copy(&mut bytes.as_ref(), &mut temporary).expect("Can't copy content to temporary file!");

    temporary
}