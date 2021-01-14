mod model;

use naday::cli_parse;
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let ctx = cli_parse(&args);

    println!("Command: {:?}", ctx.action);
}
