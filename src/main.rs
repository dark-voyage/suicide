pub mod install;
pub mod show;
pub mod update;
pub mod utils;

use clap::Parser;

/// Show suicidal quotes on shell startup
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Show random picked suicidal quote
    #[clap(short, long, value_parser)]
    show: bool,

    /// Update the database of quotes
    #[clap(short, long, value_parser)]
    update: bool,

    /// Initialize the database of quotes
    #[clap(short, long, value_parser)]
    install: bool,
}

#[tokio::main]
async fn main() {
    show::list().await
}
