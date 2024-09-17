use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use config::{ConfigError, Source, Value, ValueKind};
use std::collections::HashMap;

use crate::settings::DisplayConfig;

#[derive(Debug, Parser, Clone)]
#[command(author, no_binary_name=false, version, about, long_about = None, bin_name="jrnl", disable_help_subcommand=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    pub journal: Option<String>,

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
#[group(multiple = true, id = "search_filters", required = false)]
pub struct SearchOptions {
    #[arg(long, help = "Show entries on this date", required = false)]
    pub on: Option<String>,

    #[arg(long, action = ArgAction::SetTrue, help = "Show entries on today over the years", required = false)]
    pub today_in_history: Option<bool>,

    #[arg(
        long,
        help = "Show entries on this month of any year",
        required = false
    )]
    pub month: Option<String>,

    #[arg(long, help = "Show entries on this day of any month", required = false)]
    pub day: Option<String>,

    #[arg(long, help = "Show entries of a specific year", required = false)]
    pub year: Option<String>,

    #[arg(long, help = "Show entries after, or on, this date", required = false)]
    pub from: Option<String>,

    #[arg(long, help = "Show entries before, or on, this date", required = false)]
    pub to: Option<String>,

    #[arg(
        long,
        help = "Show entries containing specific text (put quotes around text with spaces)",
        required = false
    )]
    pub contains: Option<String>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Show only entries that match all conditions",
        required = false
    )]
    pub and: Option<bool>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Show only starred entries (marked with *)",
        required = false
    )]
    pub starred: Option<bool>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Show only entries that have at least one tag",
        required = false
    )]
    pub tagged: Option<bool>,

    #[arg(
        long,
        help = "Show a maximum of NUMBER entries",
        value_name = "NUMBER",
        required = false
    )]
    pub n: Option<u32>,

    #[arg(
        long,
        help = "If passed a string, will exclude entries with that tag. Can also be used before --starred or '--tagged' flags to invert the matching criteria of those flags",
        value_name = "TAG/FLAG",
        required = false
    )]
    pub not: Option<String>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Opens the selected entries in your configured editor",
        required = false
    )]
    pub edit: Option<bool>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Interactively deletes selected entries",
        required = false
    )]
    pub delete: Option<bool>,

    #[arg(
        long,
        value_name = "DATE",
        default_value = "now",
        required = false,
        help = "Change timestamp for selected entries"
    )]
    pub change_time: Option<String>,

    #[arg(
        long,
        value_name = "TYPE",
        required = false,
        help = "Display selected entries in an alternate format"
    )]
    pub format: Option<DisplayConfig>,

    #[arg(
        long,
        value_name = "FILENAME",
        requires = "format",
        required = false,
        help = "Write output to file instead of stdout"
    )]
    pub file: Option<String>,

    #[arg(
        long,
        required = false,
        help = "Alias for '--format tags'. Returns a list of all tags and number of occurrences"
    )]
    pub tags: Option<bool>,

    #[arg(
        long,
        required = false,
        action,
        help = "Alias for '--format short'. Show only titles or line containing the search tags"
    )]
    pub short: Option<bool>,
}

#[derive(Debug, Args, Clone)]
pub struct FormatArgs {
    format: DisplayConfig,
    #[arg(long, value_name = "FILENAME")]
    file: String,
}

// TODO: get rid of non-'--' prefixed subcommand names
#[derive(Debug, Subcommand, Clone)]
#[command(no_binary_name = true)]
pub enum Commands {
    #[command(
        long_flag = "list",
        about = "List all configured journals",
        allow_external_subcommands = true
    )]
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
    #[arg(long, required = false)]
    pub(crate) format: Option<ListFormat>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ListFormat {
    Json,
    Yaml,
}

#[derive(Debug, Args, Clone)]
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

impl Source for Cli {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut map = HashMap::new();
        for chunk in self.config_override.chunks_exact(2) {
            match chunk {
                [k, v] => match k.as_str() {
                    k if k.starts_with("color.") => {
                        let key = "color".to_string();
                        let color_section = k.strip_prefix("color.").unwrap().to_string();
                        let value = Value::new(None, ValueKind::String(v.to_string()));
                        map.entry(key)
                            .and_modify(|table| match table {
                                Value {
                                    kind: ValueKind::Table(tab),
                                    ..
                                } => {
                                    tab.insert(color_section.clone(), value.clone());
                                }
                                _ => panic!("nope"),
                            })
                            .or_insert_with(|| {
                                let mut map = HashMap::new();
                                map.insert(color_section.clone(), value.clone());
                                let vk = ValueKind::Table(map);
                                Value::new(Some(&"cli".to_owned()), vk)
                            });
                    }
                    k if k.starts_with("journals.") => {
                        let key = "journals".to_string();
                        let journal_name = k.strip_prefix("journals.").unwrap().to_string();
                        let path = Value::new(None, ValueKind::String(v.to_string()));
                        map.entry(key)
                            .and_modify(|table| match table {
                                Value {
                                    kind: ValueKind::Table(tab),
                                    ..
                                } => {
                                    tab.insert(journal_name.clone(), path.clone());
                                }
                                _ => panic!("nope"),
                            })
                            .or_insert_with(|| {
                                let mut map = HashMap::new();
                                map.insert(journal_name.clone(), path.clone());
                                let vk = ValueKind::Table(map);
                                Value::new(Some(&"cli".to_owned()), vk)
                            });
                    }
                    "encrypt" | "highlight" => match v.to_lowercase().as_str() {
                        "false" | "0" => {
                            map.insert(
                                k.to_string(),
                                Value::new(Some(&"cli".to_owned()), ValueKind::Boolean(false)),
                            );
                        }
                        "true" | "1" => {
                            map.insert(
                                k.to_string(),
                                Value::new(Some(&"cli".to_owned()), ValueKind::Boolean(true)),
                            );
                        }
                        _ => panic!("asdf"),
                    },
                    "default_minute" | "default_hour" | "linewrap" => {
                        map.insert(
                            k.to_string(),
                            Value::new(
                                Some(&"cli".to_owned()),
                                ValueKind::I64(v.parse::<i64>().expect("not a number")),
                            ),
                        );
                    }
                    _ => {
                        map.insert(
                            k.to_string(),
                            Value::new(
                                Some(&"cli".to_owned()),
                                ValueKind::String(v.clone().to_string()),
                            ),
                        );
                    }
                },
                _ => return Err(ConfigError::Message("asdfasdf".to_string())),
            }
        }
        Ok(map)
    }
}
