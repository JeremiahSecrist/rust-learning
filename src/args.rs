use clap::{Arg, ArgAction, Command};
/// Needs to be declared that it is of type clap::command 
/// enables gettings matches on the other side 
pub fn build_matches() -> clap::ArgMatches {
Command::new("pacman")
    .about("package manager utility")
    .version("5.2.1")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
        Command::new("query")
            .short_flag('Q')
            .long_flag("query")
            .about("Query the package database.")
            .arg(
                Arg::new("search")
                    .short('s')
                    .long("search")
                    .help("search locally installed packages for matching strings")
                    .conflicts_with("info")
                    .action(ArgAction::Set)
                    .num_args(1..),
            )
            .arg(
                Arg::new("info")
                    .long("info")
                    .short('i')
                    .conflicts_with("search")
                    .help("view package information")
                    .action(ArgAction::Set)
                    .num_args(1..),
            ),
    )
    // Sync subcommand
    //
    // Only a few of its arguments are implemented below.
    .subcommand(
        Command::new("sync")
            .short_flag('S')
            .long_flag("sync")
            .about("Synchronize packages.")
            .arg(
                Arg::new("search")
                    .short('s')
                    .long("search")
                    .conflicts_with("info")
                    .action(ArgAction::Set)
                    .num_args(1..)
                    .help("search remote repositories for matching strings"),
            )
            .arg(
                Arg::new("info")
                    .long("info")
                    .conflicts_with("search")
                    .short('i')
                    .action(ArgAction::SetTrue)
                    .help("view package information"),
            )
            .arg(
                Arg::new("package")
                    .help("packages")
                    .required_unless_present("search")
                    .action(ArgAction::Set)
                    .num_args(1..),
            ),
    ).get_matches()
}