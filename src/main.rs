pub mod utils;
pub mod app;
pub mod cli;
pub mod stores;
pub mod data;
pub mod sspi_client;
pub mod rustls;


use crate::app::App;

#[tokio::main]
async fn main() {
    // env_logger::builder().filter_level(log::LevelFilter::Trace).init();
    // Get args
    let args: Vec<String> = std::env::args().collect();
    
    let mut app = App::new();
    
    // Check for file argument
    if args.len() >= 2 {
        let file_path = &args[1];
        match app.before_run(file_path).await {
            Ok(should_exit) => {
                if should_exit {
                    return;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Start interactive mode
    app.run().await;
}