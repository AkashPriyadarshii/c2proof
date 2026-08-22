use clap::Parser;

fn main() {
    std::process::exit(c2proof::Cli::parse().run());
}
