use std::io::{self, Write};

use crate::core::errors::Result;
use crate::core::user_interaction_interface::UserInteractionInterface;
use async_trait::async_trait;

pub struct CliUserInteractionInterface {}

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

    async fn get_input(&self, prompt: &str) -> Result<String> {
        print!("❓ {}: ", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        Ok(input)
    }
}
