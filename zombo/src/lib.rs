use std::path::Path;

pub mod model;

#[derive(Debug)]
struct Zomboid {
    path: Path,
}

#[cfg(test)]
mod tests {
    use crate::model::Item;
    use csv;
    use std::{env::current_dir, path::PathBuf};

    fn data_path() -> PathBuf {
        let cwd = current_dir().unwrap();
        cwd.parent().unwrap().join(".data/data.csv")
    }

    #[test]
    fn csv_reader() {
        let mut r = csv::Reader::from_path(data_path()).unwrap();

        for res in r.deserialize() {
            let rec: Item = res.unwrap();
            println!("{:?}", rec);
        }
    }
}
