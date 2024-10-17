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
    List {
        #[arg(short, long)]
        take: Option<usize>,

        #[arg(short, long)]
        skip: Option<usize>,
    },
    Describe,
}

fn main() {
    let args = Args::parse();
    let mut r = csv::Reader::from_path(args.path).unwrap();
    let mut zombo = Zomboid::new(r.deserialize());
    match args.cmd {
        Command::List { take, skip } => {
            zombo.set_take(take);
            zombo.set_skip(skip);
            let table = zombo.stream().unwrap();
            println!("{table}");
        }
        Command::Describe => {
            let table = zombo.describe().unwrap();
            println!("{table}");
        }
    };
}
