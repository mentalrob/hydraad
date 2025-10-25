pub mod utils;
pub mod app;
pub mod cli;
pub mod stores;
pub mod data;


use crate::app::App;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    let mut app = App::new();
    app.run().await;
}