use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
// Small cli to retrieve wallpaper download urls and hashing information for Nix.
pub struct Opts {
    /// wallhaven id used to utilize the api.
    #[arg(
        required_unless_present = "input_file",
        default_value_t,
        hide_default_value = true
    )]
    pub id: String,

    /// what you want to call the wallpaper
    #[arg(
        required_unless_present = "input_file",
        default_value_t,
        hide_default_value = true
    )]
    pub name: String,

    /// whether the cli should be verbose with output
    #[arg(short, long, default_value_t = false)]
    pub verbose : bool,

    /// optional .toml file instead to parse instead of a wallhaven id.
    #[arg(short, long, group = "input_file")]
    pub file : Option<PathBuf>
}
