#[path = "../glupload.rs"]
mod glupload;
use glupload::*;

#[path = "../examples-common.rs"]
pub mod examples_common;

fn example_main() {
    App::new()
        .and_then(main_loop)
        .unwrap_or_else(|e| eprintln!("Error! {}", e))
}

fn main() {
    examples_common::run(example_main);
}
