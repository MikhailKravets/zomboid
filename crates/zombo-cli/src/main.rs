use clap::{Parser, Subcommand};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use zombo::{
    model::{Item, Stat},
    table::Table,
    Zomboid,
};

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

/// Read all files in `path` directory and return a Vector of [csv::Reader] objects.
/// The directory must contain only `.csv` files, otherwise the function will
/// return an error.
fn dir_to_readers(path: impl AsRef<Path>) -> io::Result<Vec<csv::Reader<fs::File>>> {
    let mut vec = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        vec.push(csv::Reader::from_path(entry.path())?);
    }
    Ok(vec)
}

/// Helper enumeration that allows handling either a single `.csv` file
/// in the path or all the files in a whole directory.
enum ZomboIter<S, D> {
    Single(Zomboid<S>),
    Dir(Zomboid<D>),
}

impl<S, D, E> ZomboIter<S, D>
where
    S: Iterator<Item = Result<Item, E>>,
    D: Iterator<Item = Result<Item, E>>,
    E: std::error::Error,
{
    fn list_table(&mut self, take: Option<usize>, skip: Option<usize>) -> Result<Table<Item>, E> {
        match self {
            Self::Single(z) => {
                z.set_take(take);
                z.set_skip(skip);
                z.stream()
            }
            Self::Dir(z) => {
                z.set_take(take);
                z.set_skip(skip);
                z.stream()
            }
        }
    }

    fn describe_table(&mut self) -> Result<Table<Stat>, E> {
        match self {
            Self::Single(z) => z.describe(),
            Self::Dir(z) => z.describe(),
        }
    }
}

fn main() {
    let args = Args::parse();

    // Using readers Vec we ensure that Readers aren't dropped
    // until iterators aren't read.
    let mut readers = Vec::<csv::Reader<fs::File>>::new();

    let mut zombo = if args.path.is_file() {
        readers.push(
            csv::Reader::from_path(args.path.as_path()).expect("Couldn't create a CSV reader."),
        );
        ZomboIter::Single(Zomboid::new(readers[0].deserialize()))
    } else {
        readers = dir_to_readers(args.path).expect("Couldn't read directory.");
        ZomboIter::Dir(Zomboid::new(
            readers.iter_mut().flat_map(|it| it.deserialize::<Item>()),
        ))
    };

    match args.cmd {
        Command::List { take, skip } => {
            let table = zombo
                .list_table(take, skip)
                .expect("Couldn't list CSV data.");
            println!("{table}");
        }
        Command::Describe => {
            let table = zombo.describe_table().expect("Couldn't describe CSV data.");
            println!("{table}");
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Args;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }

    #[test]
    fn readers_vec() {
        panic!("write later")
    }

    #[test]
    fn readers_vec_error() {
        panic!("write later")
    }
}
