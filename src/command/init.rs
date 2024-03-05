use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(name = "init", about = "Creates a config file in the current path")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
