use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::settings::DisplayConfig;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, bin_name="jrnl", disable_help_subcommand=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    pub entry: Vec<String>,

    #[command(flatten)]
    pub search: SearchOptions,

    #[arg(
        long,
        num_args = 2,
        value_names = ["CONFIG_KEY", "CONFIG_VALUE"],
        help = r#" Override configured key-value pair with CONFIG_KV_PAIR
for this command invocation only.
Examples:
- Use a different editor for this jrnl entry, call:
jrnl --config-override editor "nano"
- Override color selections
jrnl --config-override colors.body blue --config-override colors.title green"#)]
    config_override: Vec<String>,

    #[arg(
        long,
        value_name = "CONFIG_FILE_PATH",
        help = r#"Overrides default (created when first installed) config
file for this command only
Examples:
Use a work config file for this jrnl entry, call:
 jrnl --config-file /home/user1/work_config.yaml
Use a personal config file stored on a thumb drive:
 jrnl --config-file /media/user1/my-thumb-drive/personal_config.yaml"#
    )]
    pub config_file: Option<String>,
}

#[derive(Debug, Args, Clone)]
#[group(multiple = true, id = "search_filters")]
#[command(flatten = true)]
pub struct SearchOptions {
    #[arg(long, help = "Show entries on this date")]
    pub on: String,
    #[arg(long, help = "Show entries on today over the years")]
    pub today_in_history: bool,
    #[arg(long, help = "Show entries on this month of any year")]
    pub month: String,
    #[arg(long, help = "Show entries on this day of any month")]
    pub day: String,
    #[arg(long, help = "Show entries of a specific year")]
    pub year: String,
    #[arg(long, help = "Show entries after, or on, this date")]
    pub from: String,
    #[arg(long, help = "Show entries before, or on, this date")]
    pub to: String,
    #[arg(
        long,
        help = "Show entries containing specific text (put quotes around text with spaces)"
    )]
    pub contains: String,
    #[arg(long, help = "Show only entries that match all conditions")]
    pub and: bool,
    #[arg(long, help = "Show entries on this date")]
    pub starred: bool,
    #[arg(long, help = "Show entries on this date")]
    pub tagged: bool,
    #[arg(long, help = "Show entries on this date")]
    pub n: u32,
    #[arg(long, help = "Show entries on this date")]
    pub not: bool,
    // }
    //
    // #[derive(Debug, Args)]
    // #[group(multiple = true, id = "search_options")]
    // #[command(flatten = true)]
    // pub struct SearchOptions {
    #[arg(
        long,
        action,
        help = "Opens the selected entries in your configured editor"
    )]
    pub edit: bool,

    #[arg(long, action, help = "Interactively deletes selected entries")]
    pub delete: bool,

    #[arg(
        long,
        value_name = "DATE",
        default_value = "now",
        help = "Change timestamp for selected entries"
    )]
    pub change_time: Option<String>,

    #[arg(
        long,
        value_name = "TYPE",
        help = "Display selected entries in an alternate format"
    )]
    pub format: DisplayConfig,

    #[arg(
        long,
        value_name = "FILENAME",
        requires = "format",
        help = "Write output to file instead of stdout"
    )]
    pub file: String,

    #[arg(
        long,
        help = "Alias for '--format tags'. Returns a list of all tags and number of occurrences"
    )]
    pub tags: bool,

    #[arg(
        long,
        action,
        help = "Alias for '--format short'. Show only titles or line containing the search tags"
    )]
    pub short: bool,
}

#[derive(Debug, Args)]
pub struct FormatArgs {
    format: DisplayConfig,
    #[arg(long, value_name = "FILENAME")]
    file: String,
}

#[derive(Debug, Subcommand)]
// #[command(no_binary_name = true)]
pub enum Commands {
    // #[command(flatten)]
    // Entry(Vec<String>),
    #[command(long_flag = "list", about = "List all configured journals")]
    //, value_parser=["jrnl", "yaml"], default_value="jrnl")]
    List(ListArgs),
    #[command(
        long_flag = "encrypt",
        about = "Encrypt selected journal with a password"
    )]
    Encrypt,
    #[command(
        long_flag = "decrypt",
        about = "Decrypt selected journal and store it in plain text"
    )]
    Decrypt,
    #[command(long_flag = "import", about = "Import entries from another journal")]
    Import(ImportArgs),
}

#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    #[arg(long)]
    format: ListFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ListFormat {
    Jrnl,
    Yaml,
}

#[derive(Debug, Args)]
pub struct ImportArgs {
    #[arg(long, value_name = "FILENAME", default_value = "stdin")]
    file: String,
    #[arg(long, default_value = "jrnl")]
    format: ImportFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ImportFormat {
    Jrnl,
}
