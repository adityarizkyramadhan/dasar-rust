mod cli;
mod todo;

fn main() {
    let mut cli = cli::CommandLine::new();
    cli.run();
}
