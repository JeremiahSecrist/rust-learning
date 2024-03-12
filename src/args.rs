use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cliargs {
    /// example info
    pub first_arg: String,
}
