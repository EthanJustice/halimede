// std

// crates
use clap::{App, Arg, SubCommand};

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

    if let Some(_v) = app.subcommand_matches("add") {
    } else if let Some(_v) = app.subcommand_matches("init") {
    } else if let Some(_v) = app.subcommand_matches("list") {
    } else if let Some(_v) = app.subcommand_matches("remove") {
    }
}
