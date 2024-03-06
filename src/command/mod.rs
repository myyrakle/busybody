use clap::{Args, Parser};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(
        short,
        long,
        default_value = "false",
        help = "Initialize Configurations."
    )]
    pub init: bool,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(flatten)]
    pub options: ConfigOptions,
}
