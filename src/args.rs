// src/args.rs
use clap::{arg, ArgMatches, Command};

use crate::converters;

pub fn build_cli() -> Command {
    Command::new("myapp")
        .about("A CLI tool")
        .subcommand_required(true)
        .subcommand(Command::new("help").about("Prints help information"))
        .subcommand(
            Command::new("convert")
                .about("Converts a package")
                .arg(
                    arg!(--type <TYPE>)
                        .short('t')
                        .required(true)
                        .possible_values(["deb", "rpm", "tar"])
                        .about("Specifies the type of the package"),
                )
                .arg(
                    arg!(--output <OUTPUT>)
                        .short('o')
                        .required(false)
                        .default_value("./")
                        .about("Specifies the output directory"),
                ),
        )
}

pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("help", _)) => {
            println!("help here");
        }
        Some(("convert", sub_m)) => {
            let package_type = sub_m.get_one("type").unwrap();
            let output_dir = sub_m.value_of("output").unwrap_or("./");

            match package_type {
                "deb" => converters::deb::convert2deb(archive_path),
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    }
}
