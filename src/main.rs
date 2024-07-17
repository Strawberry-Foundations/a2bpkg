use std::path::Path;

mod args;
mod utils;
mod converters;

fn main() {
    converters::deb::convert2deb(Path::new("testing/tunneled-2.4.0-amd64.deb"));
}
