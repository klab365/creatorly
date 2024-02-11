use crate::core::errors::Result;

#[async_trait::async_trait]
pub trait UserInteractionInterface: Send + Sync {
    async fn print_success(&self, message: &str);

    async fn print_error(&self, message: &str);

    async fn print(&self, message: &str);

    async fn get_input(&self, prompt: &str) -> Result<String>;
}
