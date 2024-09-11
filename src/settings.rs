use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    journals: HashMap<String, JournalConfig>,
    #[serde(flatten)]
    config: CommonConfig,
    version: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            journals: HashMap::default(),
            config: CommonConfig::default(),
            version: format!("v{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
struct CommonConfig {
    colors: Option<ColorConfig>,
    default_hour: Option<u8>,
    default_minute: Option<u8>,
    display_format: Option<DisplayConfig>,
    editor: Option<String>,
    encrypt: Option<bool>,
    highlight: Option<bool>,
    indent_character: Option<char>,
    journal: Option<String>,
    linewrap: Option<LineWrapConfig>,
    tagsymbols: Option<String>,
    template: Option<TemplateConfig>,
    timeformat: Option<String>,
}

#[allow(dead_code)]
impl CommonConfig {
    fn default_hour(mut self, default_hour: u8) -> Self {
        self.default_hour = Some(default_hour);
        self
    }
    fn default_minute(mut self, default_minute: u8) -> Self {
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
    fn journal(mut self, journal: String) -> Self {
        self.journal = Some(journal);
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
            journal: None,
            linewrap: Some(LineWrapConfig::default()),
            tagsymbols: Some("#@".to_owned()),
            template: Some(TemplateConfig::default()),
            timeformat: Some("%F %r".to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum JournalConfig {
    Standard(String),
    Override(CommonConfig),
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self::Standard("".to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TemplateConfig {
    Empty(bool),
    Path(String),
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self::Empty(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LineWrapConfig {
    Auto,
    #[serde(untagged)]
    Columns(u16),
}

impl Default for LineWrapConfig {
    fn default() -> Self {
        Self::Columns(79)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct ColorConfig {
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
enum TextColor {
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
    fn colorize(&self, s: &str) -> ColoredString {
        match self {
            Self::None => s.normal(),
            Self::Black => s.black(),
            Self::Red => s.red(),
            Self::Green => s.green(),
            Self::Yellow => s.yellow(),
            Self::Blue => s.blue(),
            Self::Magenta => s.magenta(),
            Self::Cyan => s.cyan(),
            Self::White => s.white(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DisplayConfig {
    Boxed,
    Dates,
    Json,
    Markdown,
    Pretty,
    Short,
    Tags,
    Text,
    Xml,
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
    use serde_yml;
    const YAML_STR: &str = r#"
colors:
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

    #[test]
    fn test_deser_config() {
        let _: Settings = serde_yml::from_str(YAML_STR).unwrap();
    }

    #[test]
    fn test_ser_config() {
        let config = Settings::default();
        let _ = serde_yml::to_string(&config).unwrap();
    }
}
