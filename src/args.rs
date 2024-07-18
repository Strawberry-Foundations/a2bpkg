use std::path::Path;
use clap::{arg, ArgMatches, Command};

use crate::converters;

pub fn build_cli() -> Command {
    Command::new("a2bpkg")
        .about("Convert common archive types to binpkg files")
        .subcommand_required(true)
        .subcommand(
            Command::new("convert")
                .about("Converts a package")
                .arg(
                    arg!(<ARCHIVE>)
                        .required(true)
                        .help("Specifies the archive to be converted"),
                )
                .arg(
                    arg!(--type <TYPE>)
                        .short('t')
                        .required(true)
                        .value_parser(["deb", "rpm", "tar"])
                        .help("Specifies the type of the package"),
                )
                .arg(
                    arg!(--output <OUTPUT>)
                        .short('o')
                        .required(false)
                        .default_value("./")
                        .help("Specifies the output directory"),
                ),
        )
}

pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("convert", sub_m)) => {
            let archive = sub_m.get_one::<String>("ARCHIVE").unwrap().as_str();
            let package_type = sub_m.get_one::<String>("type").unwrap().as_str();
            let output_dir = sub_m
                .get_one::<String>("output")
                .map(|s| s.as_str())
                .unwrap_or("./");

            match package_type {
                "deb" => converters::deb::convert2deb(Path::new(archive), output_dir),
                "rpm" | "tar" => println!("Not implemented."),
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    }
}
