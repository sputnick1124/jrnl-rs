use clap::ValueEnum;
#[allow(unused_imports)]
use config::{Config, ConfigError, Environment, File};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use termcolor::Color;

use crate::cli::Cli;
use crate::error::{JrnlError, JrnlErrorKind, Result};
#[allow(unused_imports)]
use crate::journal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    #[serde(flatten)]
    config: CommonConfig,
    version: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            config: CommonConfig::default(),
            version: format!("v{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

#[allow(dead_code)]
impl<'a> Settings {
    pub fn configure(file: &str, cli: Cli) -> std::result::Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(file))
            .add_source(cli)
            .build()?;
        s.try_deserialize()
    }
    pub fn with_journal(mut self, journal_name: &str, journal_path: &str) -> Self {
        self.config.journal_config = Some(JournalConfigs::with_journal(journal_name, journal_path));
        self
    }
    pub fn journal_file(&'a self, journal_name: &str) -> Result<&'a str> {
        self.journal_settings(journal_name).map(|(_, f)| f)
    }
    fn journal_settings(&'a self, journal_name: &str) -> Result<(&'a CommonConfig, &'a str)> {
        self.config
            .journal_config
            .as_ref()
            .map(|configs| match configs {
                JournalConfigs::Journals(journals) => journals
                    .get(journal_name)
                    .map(|journal| {
                        let journal_file = journal.journal_file();
                        match journal {
                            JournalConfig::Override(config) => journal_file.map(|p| (config, p)),
                            JournalConfig::Standard(_) => journal_file.map(|p| (&self.config, p)),
                        }
                    })
                    .ok_or(JrnlError(JrnlErrorKind::MissingJournalConfig))?,
                _ => Err(JrnlError(JrnlErrorKind::TopLevelJournalConfig))?,
            })
            .ok_or(JrnlError(JrnlErrorKind::MissingJournalConfig))?
    }
    pub fn default_hour(&self, journal_name: &str) -> Result<i8> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .default_hour
            .or(self.config.default_hour)
            .unwrap_or_default())
    }
    pub fn default_minute(&self, journal_name: &str) -> Result<i8> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .default_minute
            .or(self.config.default_minute)
            .unwrap_or_default())
    }
    pub fn colors(&self, journal_name: &str) -> Result<ColorConfig> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config.colors.or(self.config.colors).unwrap_or_default())
    }
    pub fn display_format(&self, journal_name: &str) -> Result<DisplayConfig> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .display_format
            .or(self.config.display_format)
            .unwrap_or_default())
    }
    pub fn editor(&self, journal_name: &str) -> Result<String> {
        let (config, _) = self.journal_settings(journal_name)?;
        config
            .editor
            .clone()
            .or(self.config.editor.clone())
            .ok_or(JrnlError(JrnlErrorKind::InvalidJrnlOverrideConfig))
    }
    pub fn encrypt(&self, journal_name: &str) -> Result<bool> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config.encrypt.or(self.config.encrypt).unwrap_or_default())
    }
    pub fn highlight(&self, journal_name: &str) -> Result<bool> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .highlight
            .or(self.config.highlight)
            .unwrap_or_default())
    }
    pub fn indent_character(&self, journal_name: &str) -> Result<char> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .indent_character
            .or(self.config.indent_character)
            .unwrap_or_default())
    }
    pub fn linewrap(&self, journal_name: &str) -> Result<LineWrapConfig> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config.linewrap.or(self.config.linewrap).unwrap_or_default())
    }
    pub fn tagsymbols(&self, journal_name: &str) -> Result<String> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .tagsymbols
            .clone()
            .or(self.config.tagsymbols.clone())
            .unwrap_or_default())
    }
    pub fn template(&self, journal_name: &str) -> Result<TemplateConfig> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .template
            .clone()
            .or(self.config.template.clone())
            .unwrap_or_default())
    }
    pub fn timeformat(&self, journal_name: &str) -> Result<String> {
        let (config, _) = self.journal_settings(journal_name)?;
        Ok(config
            .timeformat
            .clone()
            .or(self.config.timeformat.clone())
            .unwrap_or_default())
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonConfig {
    colors: Option<ColorConfig>,
    default_hour: Option<i8>,
    default_minute: Option<i8>,
    display_format: Option<DisplayConfig>,
    editor: Option<String>,
    encrypt: Option<bool>,
    highlight: Option<bool>,
    indent_character: Option<char>,
    #[serde(flatten)]
    journal_config: Option<JournalConfigs>,
    linewrap: Option<LineWrapConfig>,
    tagsymbols: Option<String>,
    template: Option<TemplateConfig>,
    timeformat: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum JournalConfigs {
    Journals(IndexMap<String, JournalConfig>),
    Journal(String),
}

impl JournalConfigs {
    fn with_journal(journal_name: &str, path: &str) -> Self {
        let journal_config = JournalConfig::Override(CommonConfig {
            default_hour: None,
            default_minute: None,
            colors: None,
            display_format: None,
            editor: None,
            encrypt: None,
            highlight: None,
            indent_character: None,
            journal_config: Some(JournalConfigs::Journal(path.to_string())),
            linewrap: None,
            tagsymbols: None,
            template: None,
            timeformat: None,
        });
        let mut map = IndexMap::new();
        map.insert(journal_name.to_string(), journal_config);
        Self::Journals(map)
    }
}

#[allow(dead_code)]
impl CommonConfig {
    fn default_hour(mut self, default_hour: i8) -> Self {
        self.default_hour = Some(default_hour);
        self
    }
    fn default_minute(mut self, default_minute: i8) -> Self {
        self.default_minute = Some(default_minute);
        self
    }
    fn colors(mut self, colors: ColorConfig) -> Self {
        self.colors = Some(colors);
        self
    }
    fn display_format(mut self, display_format: DisplayConfig) -> Self {
        self.display_format = Some(display_format);
        self
    }
    fn editor(mut self, editor: String) -> Self {
        self.editor = Some(editor);
        self
    }
    fn encrypt(mut self, encrypt: bool) -> Self {
        self.encrypt = Some(encrypt);
        self
    }
    fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }
    fn indent_character(mut self, indent_character: char) -> Self {
        self.indent_character = Some(indent_character);
        self
    }
    fn journal_config(mut self, journals: JournalConfigs) -> Self {
        self.journal_config = Some(journals);
        self
    }
    fn linewrap(mut self, linewrap: LineWrapConfig) -> Self {
        self.linewrap = Some(linewrap);
        self
    }
    fn tagsymbols(mut self, tagsymbols: String) -> Self {
        self.tagsymbols = Some(tagsymbols);
        self
    }
    fn template(mut self, template: TemplateConfig) -> Self {
        self.template = Some(template);
        self
    }
    fn timeformat(mut self, timeformat: String) -> Self {
        self.timeformat = Some(timeformat);
        self
    }
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            colors: Some(ColorConfig::default()),
            default_hour: Some(9),
            default_minute: Some(0),
            display_format: None,
            editor: None,
            encrypt: Some(false),
            highlight: Some(true),
            indent_character: Some('|'),
            journal_config: None,
            linewrap: Some(LineWrapConfig::default()),
            tagsymbols: Some("#@".to_owned()),
            template: Some(TemplateConfig::default()),
            timeformat: Some("%F %r".to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum JournalConfig {
    Standard(String),
    Override(CommonConfig),
}

impl JournalConfig {
    fn journal_file(&self) -> Result<&str> {
        match self {
            Self::Standard(journal_file) => Ok(journal_file),
            Self::Override(config) => {
                if let Some(JournalConfigs::Journal(journal_file)) = config.journal_config.as_ref()
                {
                    Ok(journal_file)
                } else {
                    Err(JrnlError(JrnlErrorKind::InvalidJrnlOverrideConfig))
                }
            }
        }
    }
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self::Standard("".to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TemplateConfig {
    Empty(bool),
    Path(String),
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self::Empty(false)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LineWrapConfig {
    Auto,
    #[serde(untagged)]
    Columns(i16),
}

impl Default for LineWrapConfig {
    fn default() -> Self {
        Self::Columns(79)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ColorConfig {
    body: TextColor,
    date: TextColor,
    tags: TextColor,
    title: TextColor,
}
impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            body: TextColor::None,
            date: TextColor::Black,
            tags: TextColor::Yellow,
            title: TextColor::Cyan,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TextColor {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl TextColor {
    #[allow(dead_code)]
    pub fn get_termcolor(&self) -> Option<Color> {
        match self {
            Self::None => None,
            Self::Black => Some(Color::Black),
            Self::Red => Some(Color::Red),
            Self::Green => Some(Color::Green),
            Self::Yellow => Some(Color::Yellow),
            Self::Blue => Some(Color::Blue),
            Self::Magenta => Some(Color::Magenta),
            Self::Cyan => Some(Color::Cyan),
            Self::White => Some(Color::White),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DisplayConfig {
    Boxed,
    Dates,
    Json,
    #[serde(alias = "md")]
    Markdown,
    Pretty,
    Short,
    Tags,
    #[serde(alias = "txt")]
    Text,
    Xml,
    #[serde(alias = "yml")]
    Yaml,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self::Text
    }
}

#[cfg(test)]
mod test_config {
    use super::*;
    const YAML_STR: &str = r#"colors:
  body: none
  date: black
  tags: yellow
  title: cyan
default_hour: 9
default_minute: 0
editor: vim
encrypt: false
highlight: true
indent_character: '|'
journals:
  default:
    journal: /path/to/journal.txt
  food: ~/my_recipes.txt
  work:
    encrypt: true
    journal: ~/work.txt
linewrap: 79
tagsymbols: '%#@'
template: false
timeformat: '%F %r'
version: v4.1
"#;

    fn sample_settings() -> Settings {
        let sub_config = JournalConfig::Override(
            CommonConfig::default()
                .default_hour(4)
                .default_minute(20)
                .encrypt(true)
                .journal_config(JournalConfigs::Journal("/path/to/other.txt".to_owned())),
        );
        let mut journal_configs = IndexMap::new();
        journal_configs.insert(
            "default".to_owned(),
            JournalConfig::Standard("/path/to/default.txt".to_owned()),
        );
        journal_configs.insert("other".to_owned(), sub_config);
        let journal_config = JournalConfigs::Journals(journal_configs);
        let config = CommonConfig::default().journal_config(journal_config);
        Settings {
            config,
            ..Default::default()
        }
    }

    #[test]
    fn test_deser_config() {
        let _: Settings = serde_yml::from_str(YAML_STR).unwrap();
    }

    #[test]
    fn test_ser_config() {
        let config = Settings::default();
        let _ = serde_yml::to_string(&config).unwrap();
    }

    #[test]
    fn test_round_trip() {
        let config: Settings = serde_yml::from_str(YAML_STR).unwrap();
        let config_str = serde_yml::to_string(&config).unwrap();
        eprintln!("{config:#?}");
        assert_eq!(YAML_STR, config_str);
    }

    #[test]
    fn test_toplevel_defaults() {
        let settings = sample_settings();
        assert_eq!(settings.default_hour("default").unwrap(), 9);
        assert_eq!(settings.default_hour("other").unwrap(), 4);
        assert_eq!(settings.default_minute("default").unwrap(), 0);
        assert_eq!(settings.default_minute("other").unwrap(), 20);
        assert!(!settings.encrypt("default").unwrap());
        assert!(settings.encrypt("other").unwrap());
        assert!(settings.highlight("default").unwrap());
        assert!(settings.highlight("other").unwrap());
    }

    #[test]
    fn test_config_errors() {
        let settings = sample_settings();
        let expected_missing = JrnlErrorKind::MissingJournalConfig;
        let actual_missing = settings.journal_settings("foobar");
        if let Err(e) = actual_missing {
            assert_eq!(JrnlErrorKind::MissingJournalConfig, e.kind());
        } else {
            let kind = actual_missing.err().unwrap().kind();
            panic!("expected {expected_missing}, got {kind}");
        }

        // make standalone config from subconfig
        let (config, _) = settings.journal_settings("other").unwrap();
        let invalid_settings = Settings {
            config: config.clone(),
            ..Default::default()
        };
        let expected_toplevel = JrnlErrorKind::TopLevelJournalConfig;
        let actual_toplevel = invalid_settings.journal_settings("other");
        if let Err(e) = actual_toplevel {
            assert_eq!(expected_toplevel, e.kind());
        } else {
            let kind = actual_toplevel.err().unwrap().kind();
            panic!("expected {expected_toplevel}, got {kind}");
        }
    }
}
