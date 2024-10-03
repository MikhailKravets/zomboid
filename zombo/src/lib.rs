#[derive(Debug, serde::Deserialize)]
struct Item {
    id: u32,
    name: String,

    // Add Enum for below's two fields
    #[serde(rename = "type")]
    item_type: String,
    condition: String,
    amount: u32,
}

#[derive(Debug)]
struct Zomboid {
    data: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use crate::Item;
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
