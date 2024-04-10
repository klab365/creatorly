use super::command::GroupCommands;
use crate::core::errors::Result;

pub async fn handle_subcommand(subcommand: &dyn GroupCommands, args: &clap::ArgMatches) -> Result<()> {
    let subcommand_name = args.subcommand_name();
    if subcommand_name.is_none() {
        return Err("No subcommand entered".into());
    }

    let subcommand_name = subcommand_name.unwrap();
    let subcommand_args = args.subcommand_matches(subcommand_name);
    if subcommand_args.is_none() {
        return Err("No subcommand found".into());
    }

    let subcommand_args = subcommand_args.unwrap();
    for command in subcommand.get_commands().iter() {
        if command.get_name() == subcommand_name {
            return command.execute(subcommand_args).await;
        }
    }

    Ok(())
}
