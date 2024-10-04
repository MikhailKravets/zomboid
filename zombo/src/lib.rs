use core::str;
use model::Item;
use table::Table;

pub mod model;
pub mod table;

#[derive(Debug)]
pub struct Zomboid<T> {
    it: T,
    _take: Option<usize>,
    _skip: Option<usize>,
}

impl<T> Zomboid<T> {
    pub fn set_take(&mut self, v: Option<usize>) {
        self._take = v;
    }

    pub fn set_skip(&mut self, v: Option<usize>) {
        self._skip = v;
    }
}

impl<T, E> Zomboid<T>
where
    T: Iterator<Item = Result<Item, E>>,
    E: std::error::Error,
{
    pub fn new(it: T) -> Self {
        Self {
            it,
            _take: None,
            _skip: None,
        }
    }

    pub fn stream(&mut self) -> Result<Table<&str>, E> {
        let it = &mut self.it;
        let items: Result<Vec<Item>, E> = it
            .skip(self._skip.unwrap_or(0))
            .take(self._take.unwrap_or(usize::MAX))
            .collect();
        Ok(Table::new(items?).with_header(vec!["ID", "NAME", "TYPE", "CONDITION", "AMOUNT"]))
    }

    pub fn describe(&mut self) -> Result<Table<&str>, E> {
        // TODO: this method should describe the Iterator statistics
        //      1. Percentage of items of each condition
        //      2. ...
        //      3. Make Table header owned instead of &'static str?
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::Item, Zomboid};
    use std::{env::current_dir, path::PathBuf};

    fn data_path() -> PathBuf {
        let cwd = current_dir().unwrap();
        cwd.parent().unwrap().join(".data/data.csv")
    }

    #[ignore = "experiment"]
    #[test]
    fn csv_reader() {
        let mut r = csv::Reader::from_path(data_path()).unwrap();

        for res in r.deserialize() {
            let rec: Item = res.unwrap();
            println!("{:?}", rec);
        }
    }

    #[test]
    fn zomboid_csv() {
        let mut r = csv::Reader::from_path(data_path()).unwrap();
        let mut z = Zomboid::new(r.deserialize());

        z.set_take(Some(2));
        z.set_skip(Some(2));

        assert_eq!(z._skip, Some(2));
        assert_eq!(z._take, Some(2));

        let table = z.stream().unwrap();
        let data = table.as_data();

        assert_eq!(data.len(), 2);
        assert_eq!(data[0].id, 2);
        assert_eq!(data[1].id, 3);

        println!("{}", table);
    }
}
