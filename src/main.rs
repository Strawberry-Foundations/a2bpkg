mod args;
mod utils;
mod converters;

fn main() {
    let matches = args::build_cli().get_matches();
    args::handle_matches(matches);
}
