use clap::{Parser, Subcommand};
use std::path::PathBuf;
use zombo::Zomboid;

#[derive(Parser, Debug)]
#[command(name = "Zomboid CLI")]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    List { take: usize, skip: usize },
    Describe,
}

fn main() {
    let args = Args::parse();
    // TODO: read data from args.path. Should it be directory or file?
    // TODO: create csv iterator; chain in case of directory. Pass to Zomboid
    // TODO: match command and do action
    // let zombo = Zomboid::new();
    println!("{args:?}")
}
