use std::{env, fs};
use std::path::Path;
use libspkg::binpkg::BinPkg;
use libspkg::binpkg::metadata::Metadata;
use debber::Archive;
use crate::utils::package::test_package;

pub fn convert2deb(archive_path: &Path, output_path: &str) {
    let deb = Archive::new(archive_path).unwrap();
    let control_map = deb.control_map().unwrap();

    let name = control_map.get("Package").unwrap();
    let version = control_map.get("Version").unwrap();
    let description = control_map.get("Description").unwrap();
    let maintainer = control_map.get("Maintainer").unwrap().as_str();
    let arch = match control_map.get("Architecture").unwrap().as_str() {
        "amd64" => "x86_64",
        "i386" | "x32" => "x86",
        "arm64" => "aarch64",
        _ => "unknown"
    };

    let working_directory = format!(
        "{}/a2bpkg/{}+{}",
        env::temp_dir().to_str().expect("Failed to convert temp_dir to str"),
        name,
        version
    );
    let output = if output_path == "./" {
        format!("./{}-{}-{}.binpkg", name, version, arch)
    } else {
        output_path.to_string()
    };

    fs::create_dir_all(&working_directory).expect("Error while creating temporary directory");
    deb.data_extract(&working_directory).expect("Error while extracting .deb Archive");

    println!("{}", output);

    match BinPkg::create(
        Metadata::new(
            name,
            name,
            version,
            description,
            arch,
            maintainer
        ),
        working_directory,
        &output
    ) {
        Ok(..) => println!("Successfully created package!"),
        Err(err) => println!("Error while creating package: {err}")
    }

    test_package(output);
}
