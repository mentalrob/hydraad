pub mod utils;
pub mod app;
pub mod cli;
pub mod stores;
pub mod data;

use winston::{format::colorize, log, transports::stdout, Logger};

use crate::{app::App, utils::cli_utils::{clear_screen, print_banner}};

#[tokio::main]
async fn main() {
    let logger = Logger::builder()
        .level("info")
        .format(colorize())
        .transport(stdout())
        .build();

    winston::init(logger);

    let mut app = App::new();
    app.run().await;
}