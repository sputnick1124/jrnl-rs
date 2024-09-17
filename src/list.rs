// use std::path::PathBuf;

use crate::cli::{Cli, ListArgs, ListFormat};
use crate::settings::{JournalConfigs, Settings};
use serde_json::json;
use serde_yml::Value;

pub fn list(args: &ListArgs, cli: &Cli, settings: &Settings, config_file: &str) {
    let journal_configs = settings.get_journals();
    let config_file = config_file.clone();

    // if let Some(_journal_name) = cli.journal.clone() {}
    match args.format {
        Some(ListFormat::Json) => list_json(&journal_configs, config_file),
        Some(ListFormat::Yaml) => list_yaml(&journal_configs, config_file),
        None => todo!(),
    }
}

fn list_json(journal_map: &JournalConfigs, config_file: &str) {
    let mut j = json!({
        "config_path": config_file,
    });
    j.as_object_mut()
        .unwrap()
        .append(json!(journal_map).as_object_mut().unwrap());

    println!("{}", j.to_string());
}

fn list_yaml(journal_map: &JournalConfigs, config_file: &str) {
    let mut map = serde_yml::Mapping::new();
    let journal_value = serde_yml::to_value(journal_map).expect("error serializing this struct");
    map.insert(Value::String("config_path".into()), config_file.into());
    if let Value::Tagged(val) = journal_value {
        map.insert(Value::String(val.tag.string.into()), val.value);
    }
    let mut value = serde_yml::Value::Mapping(map);
    // value.apply_merge();
    println!("{}", serde_yml::to_string(&value).unwrap())
}
