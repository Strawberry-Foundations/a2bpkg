use std::{env, fs};
use std::path::Path;
use libspkg::binpkg::BinPkg;
use libspkg::binpkg::metadata::Metadata;
use debber::Archive;
use crate::utils::package::test_package;

pub fn convert2deb(archive_path: &Path) {
    let deb = Archive::new(archive_path).unwrap();
    let control_map = deb.control_map().unwrap();
    let arch = match control_map.get("Architecture").unwrap().as_str() {
        "amd64" => "x86_64",
        "i386" | "x32" => "x86",
        "arm64" => "aarch64",
        _ => "unknown"
    };

    let working_directory = format!("{}a2bpkg/{}+{}",
                                    env::temp_dir().to_str().unwrap(),
                                    control_map.get("Package").unwrap(),
                                    control_map.get("Version").unwrap()
    );
    let output = format!("./{}-{}-{}.binpkg",
                         control_map.get("Package").unwrap(),
                         control_map.get("Version").unwrap(),
                         arch
    );

    fs::create_dir_all(&working_directory).expect("Error while creating temporary directory");
    deb.data_extract(&working_directory).expect("Error while extracting .deb Archive");

    match BinPkg::create(
        Metadata::new(
            control_map.get("Package").unwrap().as_str(),
            control_map.get("Package").unwrap().as_str(),
            control_map.get("Version").unwrap().as_str(),
            control_map.get("Description").unwrap().as_str(),
            arch,
            control_map.get("Maintainer").unwrap().as_str()
        ),
        working_directory,
        &output
    ) {
        Ok(..) => println!("Successfully created package!"),
        Err(err) => println!("Error while creating package: {err}")
    }

    test_package(output);
}
