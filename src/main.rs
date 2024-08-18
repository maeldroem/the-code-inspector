use clap::Parser;

mod cli;

fn main() {
    cli::Args::parse();
}