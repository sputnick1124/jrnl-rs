#![feature(iter_collect_into)]

// use std::path::PathBuf;

use clap::Parser;
use config::{Config, File, FileFormat};
use directories::ProjectDirs;

mod cli;
mod decrypt;
mod encrypt;
mod entry;
mod error;
mod import;
mod journal;
mod list;
mod settings;

use cli::{Cli, Commands};
use settings::Settings;

fn handle_subcommand(cli: &Cli, settings: &Settings, config_file: &str) {
    match cli.command.clone() {
        Some(Commands::Encrypt) => encrypt::encrypt(),
        Some(Commands::Decrypt) => decrypt::decrypt(),
        Some(Commands::List(args)) => list::list(&args, cli, settings, config_file),
        Some(Commands::Import(args)) => import::import(&args),
        None => (),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::parse();

    // println!(
    // "CLI:\n{cli:#?} => {:#?}",
    // <dyn config::Source>::collect(&cli)
    // );
    let project_base = ProjectDirs::from("", "", "jrnl").unwrap();
    let conffile = if cli.config_file.clone().is_some() {
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
            std::fs::write(&conffile, &yaml)?;
            Config::builder()
                .add_source(File::from_str(&yaml, FileFormat::Yaml))
                .add_source(cli.clone())
                .build()?
                .try_deserialize()?
        }
        err => err?,
    };
    // println!("Settings:\n{settings:#?}");

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
    // println!(
    //     "CLI:\n{cli:#?} => {:#?}",
    //     <dyn config::Source>::collect(&cli)
    // );
    handle_subcommand(&cli, &settings, &conffile.clone());
    let mut file = std::fs::File::open(journal_file).expect("File open failed");
    let _journal = journal::Journal::from_file(&journal_name, &mut file);
    // println!("{:#?}", journal);
    Ok(())
}
