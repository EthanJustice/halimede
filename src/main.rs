// std
use std::fs::{create_dir, copy};

// crates
use clap::{App, Arg, SubCommand};
use directories::BaseDirs;
use walkdir::WalkDir;

// local

fn main() {
    let app = App::new("halimede")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new template")
                .arg(
                    Arg::with_name("local")
                        .short("l")
                        .help("Add a local directory as a template.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("template")
                        .short("t")
                        .help("The name of the templte to use")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Start a new project")
                .arg(
                    Arg::with_name("template")
                        .short("t")
                        .help("The name of the template to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("The output directory")
                        .default_value(".")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List all available templates."))
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove a template")
                .arg(
                    Arg::with_name("template")
                        .short("t")
                        .required(true)
                        .help("The name of the template to remove."),
                ),
        )
        .get_matches();

    let basedir = BaseDirs::new().expect("Faileed to save template.");

    if let Some(v) = app.subcommand_matches("add") {
        let template = v
            .value_of("template")
            .expect("Failed to provide a template name, aborting...");
        let data_dir = basedir.data_dir();

        if data_dir.join("halimede").exists() == true {
            if data_dir.join("halimede").join("templates").exists() == true {
                create_dir(data_dir.join(format!("/halimede/templates/{}", template)))
                    .expect("Failed to create a directory to save the template.");


            } else {
                create_dir(data_dir.join("halimede/templates"))
                    .expect("Failed to generate template storage space.");
            }
        } else {
            create_data_dir().expect("Failed to create storage for templates.");
        }
    } else if let Some(_v) = app.subcommand_matches("init") {
    } else if let Some(_v) = app.subcommand_matches("list") {
    } else if let Some(_v) = app.subcommand_matches("remove") {
    }
}

fn create_data_dir() -> Result<(), std::io::Error> {
    let basedir = BaseDirs::new().expect("Faileed to save template.");
    let data_dir = basedir.data_dir();
    create_dir(data_dir.join("halimede"))
}
