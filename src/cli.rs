use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[arg(
        required_unless_present = "input_file",
        default_value_t,
        hide_default_value = true
    )]
    pub id: String,

    #[arg(
        required_unless_present = "input_file",
        default_value_t,
        hide_default_value = true
    )]
    pub name: String,

    #[arg(short, long, default_value_t = false)]
    pub verbose : bool,

    #[arg(short, long, group = "input_file")]
    pub file : Option<PathBuf>
}
