mod cli;

pub fn cli_parse(args: &[String]) -> cli::RunContext {
    cli::RunContext::new(args)
}
