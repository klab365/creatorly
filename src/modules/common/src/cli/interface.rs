use crate::core::errors::Result;

#[async_trait::async_trait]
/// Represents a command that can be executed through a command-line interface.
pub trait ICommand: Send + Sync {
    /// Returns the name of the command.
    fn get_name(&self) -> &'static str;

    /// Executes the command with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The command-line arguments passed to the command.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the command executed successfully, or an error message as `Err(String)` if there was an error.
    async fn execute(&self, args: &clap::ArgMatches) -> Result<()>;

    /// Registers the command with the given command-line interface.
    ///
    /// # Arguments
    ///
    /// * `cli` - The command-line interface to register the command with.
    ///
    /// # Returns
    ///
    /// Returns the modified command-line interface with the command registered.
    fn register_cli(&self, cli: clap::Command) -> clap::Command;
}

pub trait IGroupCommands: Send + Sync {
    fn get_commands(&self) -> Vec<Box<dyn ICommand>>;
}
