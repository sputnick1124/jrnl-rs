#![feature(iter_collect_into)]
use clap::Parser;
use config::{Config, File, FileFormat};
use directories::ProjectDirs;

mod cli;
mod entry;
mod error;
mod journal;
mod settings;

use cli::Cli;
use settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::parse();

    let project_base = ProjectDirs::from("", "", "jrnl").unwrap();
    let conffile = if cli.config_file.is_some() {
        cli.config_file.clone().unwrap()
    } else {
        project_base
            .config_local_dir()
            .join("jrnl.yaml")
            .to_str()
            .unwrap()
            .to_owned()
    };
    let settings = match Settings::configure(&conffile, cli.clone()) {
        Ok(s) => s,
        _err if !std::fs::exists(&conffile)? => {
            let journal_file = project_base
                .data_local_dir()
                .join("journal.txt")
                .to_str()
                .unwrap()
                .to_owned();
            let s = Settings::default().with_journal("default", &journal_file);
            std::fs::create_dir_all(project_base.config_local_dir())?;
            let yaml = serde_yml::to_string(&s)?;
            std::fs::write(conffile, &yaml)?;
            Config::builder()
                .add_source(File::from_str(&yaml, FileFormat::Yaml))
                .add_source(cli.clone())
                .build()?
                .try_deserialize()?
        }
        err => err?,
    };

    let mut journal_name = cli.clone().journal.unwrap_or("default".to_owned());
    let journal_file = match settings.journal_file(&journal_name) {
        Ok(journal_file) => journal_file,
        Err(error::JrnlError(error::JrnlErrorKind::MissingJournalConfig)) => {
            cli.entry.insert(0, journal_name);
            cli.journal = None;
            journal_name = "default".to_owned();
            settings.journal_file(&journal_name)?
        }
        err => err?,
    };
    let mut file = std::fs::File::open(journal_file).expect("File open failed");
    let journal = journal::Journal::from_file(&journal_name, &mut file);
    Ok(())
}
