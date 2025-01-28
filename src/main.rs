use args::Cli;
use clap::Parser;

pub mod args;


fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
