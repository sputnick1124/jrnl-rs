#![feature(iter_collect_into)]
use clap::Parser;
use directories::ProjectDirs;

mod cli;
mod entry;
mod error;
mod journal;
mod settings;

use cli::Cli;
use settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let project_base = ProjectDirs::from("", "", "jrnl").unwrap();
    let conffile = if cli.config_file.is_some() {
        cli.config_file.unwrap()
    } else {
        project_base
            .config_dir()
            .join("jrnl.yaml")
            .to_str()
            .unwrap()
            .to_owned()
    };
    // let settings = Config::builder()
    //     .add_source(config::File::with_name(&conffile))
    //     .build()
    //     .unwrap();
    let settings = Settings::configure(&conffile)?;
    println!("Settings:\n{:#?}", settings);

    let journal_name = "default";
    let journal_file = settings.journal_file(journal_name)?;
    // let journal_name: String;
    // let journal_file: String;
    // if let Ok(journal_table) = settings.get_table("journals") {
    //     (journal_name, journal_file) = match cli.entry.get(0) {
    //         Some(journal_name) if journal_table.contains_key(journal_name) => (
    //             journal_name.to_owned(),
    //             journal_table[journal_name].clone().into_table().unwrap()["journal"]
    //                 .clone()
    //                 .into_string()
    //                 .unwrap()
    //                 .to_owned(),
    //         ),
    //         Some(_) if journal_table.contains_key("default") => (
    //             "default".to_owned(),
    //             journal_table["default"].clone().into_table().unwrap()["journal"]
    //                 .clone()
    //                 .into_string()
    //                 .unwrap()
    //                 .to_owned(),
    //         ),
    //         _ => (
    //             "default".to_owned(),
    //             project_base
    //                 .data_dir()
    //                 .join("journal.txt")
    //                 .to_str()
    //                 .unwrap()
    //                 .to_owned(),
    //         ),
    //     }
    // } else {
    //     journal_name = "default".to_owned();
    //     journal_file = project_base
    //         .data_dir()
    //         .join("journal.txt")
    //         .to_str()
    //         .unwrap()
    //         .to_owned();
    // }
    let mut file = std::fs::File::open(journal_file).expect("File open failed");
    let journal = journal::Journal::from_file(&journal_name, &mut file);
    println!("{:#?}", journal);
    Ok(())
}
