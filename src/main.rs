use human_panic::setup_panic;
use std::env;

fn main() {
    setup_panic!();

    let ctx = naday::cli_parse(env::args());

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    if let Err(msg) = naday::run(ctx) {
        eprintln!("Error: {}", msg);
    }
}
