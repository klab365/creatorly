use crate::core::errors::Result;

#[async_trait::async_trait]
pub trait UserInteractionInterface: Send + Sync {
    async fn print_success(&self, message: &str);

    async fn print_error(&self, message: &str);

    async fn print(&self, message: &str);

    async fn get_input(&self, prompt: &str, default: &str) -> Result<String>;

    async fn get_selection(&self, prompt: &str, choices: &[String]) -> Result<String>;
}
