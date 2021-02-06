use std::env;

fn main() {
    let ctx = naday::cli_parse(env::args());
    env_logger::init();

    if let Err(msg) = naday::run(ctx) {
        eprintln!("Error: {}", msg);
    }
}
