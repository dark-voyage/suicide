use owo_colors::OwoColorize;

pub fn exit(message: &str) {
    println!("{}", message.bold().bright_red());
    std::process::exit(1);
}