use crate::core::errors::Result;
use crate::core::user_interaction_interface::UserInteractionInterface;
use async_trait::async_trait;
use dialoguer::{
    console::{style, Style},
    theme::ColorfulTheme,
    Input, Select,
};

pub struct CliUserInteractionInterface {}

impl CliUserInteractionInterface {
    fn get_theme(&self) -> ColorfulTheme {
        ColorfulTheme {
            defaults_style: Style::new().for_stderr().cyan(),
            prompt_style: Style::new().for_stderr().bold(),
            prompt_prefix: style("?".to_string()).for_stderr().red(),
            prompt_suffix: style("›".to_string()).for_stderr().black().bright(),
            success_prefix: style("✔".to_string()).for_stderr().green(),
            success_suffix: style("".to_string()).for_stderr().black().bright(),
            error_prefix: style("✘".to_string()).for_stderr().red(),
            error_style: Style::new().for_stderr().red(),
            hint_style: Style::new().for_stderr().black().bright(),
            values_style: Style::new().for_stderr().green(),
            active_item_style: Style::new().for_stderr().cyan(),
            inactive_item_style: Style::new().for_stderr(),
            active_item_prefix: style("❯".to_string()).for_stderr().green(),
            inactive_item_prefix: style(" ".to_string()).for_stderr(),
            checked_item_prefix: style("✔".to_string()).for_stderr().green(),
            unchecked_item_prefix: style("⬚".to_string()).for_stderr().magenta(),
            picked_item_prefix: style("❯".to_string()).for_stderr().green(),
            unpicked_item_prefix: style(" ".to_string()).for_stderr(),
            #[cfg(feature = "fuzzy-select")]
            fuzzy_cursor_style: Style::new().for_stderr().black().on_white(),
            #[cfg(feature = "fuzzy-select")]
            fuzzy_match_highlight_style: Style::new().for_stderr().bold(),
        }
    }
}

#[async_trait]
impl UserInteractionInterface for CliUserInteractionInterface {
    async fn print_success(&self, message: &str) {
        println!("✅ {}", message);
    }

    async fn print_error(&self, message: &str) {
        println!("❌ {}", message);
    }

    async fn print(&self, message: &str) {
        println!("{}", message);
    }

    async fn get_input(&self, prompt: &str, default: &str) -> Result<String> {
        let theme = self.get_theme();
        let input = Input::with_theme(&theme)
            .with_prompt(prompt)
            .with_initial_text(default)
            .interact_text()
            .unwrap();

        Ok(input)
    }

    async fn get_selection(&self, prompt: &str, choices: &[String]) -> Result<String> {
        let theme = self.get_theme();
        let selection = Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(choices)
            .default(0)
            .interact()
            .unwrap();

        Ok(choices[selection].to_string())
    }
}
