use clap::Parser;

use crate::{app::App, cli::commands::Cli};
pub struct CommandManager;

impl CommandManager {
    pub async fn handle_command(app: &mut App, line: &str) -> Result<bool, String> {
        let args = shlex::split(line).ok_or("error: can't parse args")?;
        let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;

        cli.handle_command(app).await
    }
}