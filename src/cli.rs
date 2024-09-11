use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub entry: Vec<String>,

    #[arg(long, action)]
    pub edit: bool,

    #[arg(long, action)]
    pub delete: bool,

    #[arg(long, action)]
    pub short: bool,

    #[arg(long)]
    pub change_time: Option<String>,

    #[arg(long)]
    pub format: Option<String>,

    #[arg(long)]
    pub config_file: Option<String>,
}
