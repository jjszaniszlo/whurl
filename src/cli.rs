use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Opts {
    #[arg(
        default_value_t, 
        hide_default_value = true
    )]
    pub url: String,
}
