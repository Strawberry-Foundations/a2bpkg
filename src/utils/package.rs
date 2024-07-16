use libspkg::binpkg::BinPkg;
use libspkg::binpkg::err::BinPkgError;

pub fn test_package(output: String) {
    println!("Testing package...");

    let package = match BinPkg::read(output) {
        Ok(package) => {
            println!("Package read test passed!");
            Ok(package)
        },
        Err(err) => {
            let e: &BinPkgError = err.downcast_ref().unwrap();
            println!("{e:?}");
            println!("Error while reading package: {err}");
            println!("Package read test failed!");
            Err(err)
        }
    };

    let package = match package {
        Ok(pkg) => pkg,
        Err(_) => {
            return;
        }
    };

    println!("Reading metadata...");
    println!("Name: {}", package.metadata.name);
    println!("Description: {}", package.metadata.description);
    println!("Version: {}", package.metadata.version);
    println!("Arch: {}", package.metadata.architecture);
    println!("Done!");
}
