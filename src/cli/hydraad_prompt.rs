use std::borrow::Cow;

use colored::Colorize;
use reedline::{Prompt, PromptHistorySearchStatus};

pub struct HydraAdPrompt {
    dc_name: Option<String>,
    
}

impl HydraAdPrompt {
    pub fn new() -> Self {
        Self {
            dc_name: None,
        }
    }
    
    pub fn set_dc_name(&mut self, dc_name: Option<String>) {
        self.dc_name = dc_name;
    }
}

impl Prompt for HydraAdPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<'_, str> {
        let prompt_left = if let Some(dc_name) = &self.dc_name {
            format!("{}{}{} {} ", "[".bright_white(), dc_name.yellow(), "]".bright_white(), "hydraad".bright_white())
        } else {
            format!("{} ","hydraad".bright_white())
        };
        Cow::Owned(prompt_left)
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, prompt_mode: reedline::PromptEditMode) -> std::borrow::Cow<'_, str> {
        let indicator = "$ ".yellow().to_string();
        Cow::Owned(indicator)
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed("::: ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<'_, str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        // NOTE: magic strings, given there is logic on how these compose I am not sure if it
        // is worth extracting in to static constant
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}