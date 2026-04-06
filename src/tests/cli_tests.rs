use clap::Parser;

use crate::{Cli, Commands};

#[test]
fn test_cli_parses_without_subcommand() {
    let cli = Cli::try_parse_from(["ym2151-tone-editor", "--verbose"]).unwrap();
    assert!(cli.verbose);
    assert_eq!(cli.command, None);
}

#[test]
fn test_cli_parses_check_subcommand() {
    let cli = Cli::try_parse_from(["ym2151-tone-editor", "check"]).unwrap();
    assert_eq!(cli.command, Some(Commands::Check));
}

#[test]
fn test_cli_parses_update_subcommand() {
    let cli = Cli::try_parse_from(["ym2151-tone-editor", "update"]).unwrap();
    assert_eq!(cli.command, Some(Commands::Update));
}
