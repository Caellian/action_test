use std::path::PathBuf;

use ghactions::group;

pub mod action;
pub mod state;

fn main() {
    env_logger::init();

    let event = std::env::var("GITHUB_EVENT_PATH").expect("GitHub event not specified");
    let event = PathBuf::from(event);

    let event = std::fs::read_to_string(event).expect("missing event data");
    group!("Debugging");
    println!("Event data :: {event}");
    // let token = std::env::var("GITHUB_TOKEN").expect("github token not provided");
}
