use clap::{Parser, Subcommand};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use zombo::{model::Item, Zomboid};

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

fn readers(path: impl AsRef<Path>) -> io::Result<Vec<csv::Reader<fs::File>>> {
    let mut vec = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        vec.push(csv::Reader::from_path(entry.path())?);
    }
    Ok(vec)
}

fn main() {
    let args = Args::parse();
    let mut r = csv::Reader::from_path(args.path.as_path()).unwrap();
    let mut zombo = Zomboid::new(r.deserialize());
    let mut rdrs = readers(args.path).unwrap();

    // TODO: organize the code adequatelly.
    //       Check if is_dir()
    let mut zombo1 = Zomboid::new(rdrs.iter_mut().flat_map(|it| it.deserialize::<Item>()));
    // vec.iter_mut().flat_map(|it| it.deserialize::<Item>())
    match args.cmd {
        Command::List { take, skip } => {
            zombo.set_take(take);
            zombo.set_skip(skip);
            let table = zombo1.stream().unwrap();
            println!("{table}");
        }
        Command::Describe => {
            let table = zombo1.describe().unwrap();
            println!("{table}");
        }
    };
}
