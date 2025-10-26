use log::error;
use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, FileBackedHistory, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal
};

use crate::{
    cli::{command_manager::CommandManager, commands::Cli, hydraad_prompt::HydraAdPrompt},
    data::{Credential, DomainController},
    stores::{credentials_store::CredentialsStore, domain_controller_store::DomainControllerStore},
    utils::cli_utils::{clear_screen, list_all_commands},
};

pub struct App {
    prompt: HydraAdPrompt,
    pub domain_controller_storage: DomainControllerStore,
    pub credential_storage: CredentialsStore,
    pub current_used_dc: Option<DomainController>,
    pub current_used_creds: Option<Credential>,
}

impl App {
    pub fn new() -> App {
        App {
            prompt: HydraAdPrompt::new(),
            domain_controller_storage: DomainControllerStore::new(),
            credential_storage: CredentialsStore::new(),
            current_used_dc: None,
            current_used_creds: None,
        }
    }

    pub fn dc_storage(&mut self) -> &mut DomainControllerStore {
        &mut self.domain_controller_storage
    }

    pub fn credential_storage(&mut self) -> &mut CredentialsStore {
        &mut self.credential_storage
    }

    pub fn set_current_dc(&mut self, dc: Option<DomainController>) {
        if let Some(ref dc) = dc {
            self.prompt.set_dc_name(Some(dc.domain_name.clone()));
        } else {
            self.prompt.set_dc_name(None);
        }
        self.current_used_dc = dc;
    }

    pub fn set_current_creds(&mut self, creds: Option<Credential>) {
        if let Some(ref creds) = creds {
            self.prompt.set_credential_name(Some(format!(
                "{}:{}",
                creds.username,
                creds.auth_data_type()
            )));
        } else {
            self.prompt.set_credential_name(None);
        }
        self.current_used_creds = creds;
    }

    pub fn get_current_context(&self) -> Result<(DomainController, Credential), String> {
        if let Some(dc) = self.current_used_dc.clone() {
            if let Some(creds) = self.current_used_creds.clone() {
                return Ok((dc, creds));
            }
        }
        Err("Please set a domain controller and a credential".to_string())
    }

    pub async fn before_run(&mut self, file_path: &str) -> Result<bool, String> {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                println!("Executing commands from file: {}", file_path);
                for (line_num, line) in content.lines().enumerate() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue; // Skip empty lines and comments
                    }
                    
                    println!("Executing: {}", line);
                    match crate::cli::command_manager::CommandManager::handle_command(self, line).await {
                        Ok(should_exit) => {
                            if should_exit {
                                println!("Exit command encountered at line {}", line_num + 1);
                                return Ok(true); // Should exit
                            }
                        }
                        Err(e) => {
                            return Err(format!("Error executing line {}: {}", line_num + 1, e));
                        }
                    }
                }
                println!("File execution completed. Starting interactive mode...");
                Ok(false) // Don't exit, continue to interactive mode
            }
            Err(e) => {
                Err(format!("Error reading file '{}': {}", file_path, e))
            }
        }
    }

    pub async fn run(&mut self) {
        clear_screen();

        let completer = Box::new(DefaultCompleter::new_with_wordlen(
            list_all_commands::<Cli>(),
            2,
        ));
        // Use the interactive menu to select options from the completer
        let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
        // Set up the required keybindings
        let mut keybindings = default_emacs_keybindings();
        keybindings.add_binding(
            KeyModifiers::NONE,
            KeyCode::Tab,
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::Menu("completion_menu".to_string()),
                ReedlineEvent::MenuNext,
            ]),
        );
        let history = Box::new(
            FileBackedHistory::with_file(30, "history.txt".into())
                .expect("Error configuring history with file"),
        );

        let edit_mode = Box::new(Emacs::new(keybindings));

        let mut line_editor = Reedline::create()
            .with_completer(completer)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
            .with_edit_mode(edit_mode)
            .with_ansi_colors(true)
            .with_history(history);

        loop {
            let sig = line_editor.read_line(&self.prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    if buffer.trim().is_empty() {
                        continue;
                    }
                    match CommandManager::handle_command(self, buffer.as_str()).await {
                        Ok(true) => break,
                        Ok(false) => continue,
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("Aborted!");
                    break;
                }
                Err(err) => {
                    println!("{}",err.to_string());
                }
            }
        }
    }
}
