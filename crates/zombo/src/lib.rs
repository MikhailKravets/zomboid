//! Zomboid library provides a set of tools to read and visualize the accountant
//! data provided by the survivalists.
//!
//! Main [`Zomboid`] struct is generic over Iterator that represents the data.
//! So, it can process data of any format as long as it's an Iterator.
//!
//! Also, you may [`Zomboid::stream`] method as long as iterator is not fully consumed.
//!
//! # Examples
//!
//! [`Zomboid`] accepts a generic parameter `T: Iterator<Item = Result<Item, E>>`
//! where [`model::Item`] is a serde deserializable object.
//!
//! ## CSV
//!
//! Let's read data from `.csv` and stream it with [`Zomboid`].
//!
//! ```no_run
//! use std::error::Error;
//! use zombo::model::Item;
//! use zombo::table::Table;
//! use zombo::Zomboid;
//!
//! fn read_and_print() -> Result<Table<Item>, Box<dyn Error>> {
//!     let mut r = csv::Reader::from_path("path/to/data.csv")?;
//!     let mut z = Zomboid::new(r.deserialize());
//!
//!     // z.stream() returns a Table that we can print to terminal
//!     let table = z.stream()?;
//!
//!     Ok(table)
//! }
//! ```
//!
//! Default implementation of [`Zomboid::stream`] fully consumes the iterator.
//! We can use [`Zomboid::set_take`] and [`Zomboid::set_skip`] to create a paginated
//! behaviour of the `stream`.
//!
//! ```no_run
//! use std::error::Error;
//! use zombo::model::Item;
//! use zombo::table::Table;
//! use zombo::Zomboid;
//!
//! fn read_page() -> Result<Table<Item>, Box<dyn Error>> {
//!     let mut r = csv::Reader::from_path("path/to/data.csv")?;
//!     let mut z = Zomboid::new(r.deserialize());
//!     z.set_take(Some(10));
//!     z.set_skip(Some(5));
//!
//!     Ok(z.stream()?)
//! }
//!
//! ```
//!
//! The code above will make `z` object to take next 10 items skipping the first 5 ones.
use model::{Item, Stat};
use std::collections::HashMap;
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
    /// Set how many items to take on next [`Zomboid::stream`] call
    pub fn set_take(&mut self, v: Option<usize>) {
        self._take = v;
    }

    /// Set how many items to skip on next [`Zomboid::stream`] call
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

    /// Consumes iterator of Items and builds a table.
    ///
    /// Amount of items to take and skip can be managed
    /// by [`Zomboid::set_take`] and [`Zomboid::set_skip`].
    ///
    /// # Return
    ///
    /// Method returns [`Result<T, E>`] where `T` is [`table::Table<Item>`].
    pub fn stream(&mut self) -> Result<Table<Item>, E> {
        let items: Result<Vec<Item>, E> = self
            .it
            .by_ref()
            .skip(self._skip.unwrap_or(0))
            .take(self._take.unwrap_or(usize::MAX))
            .collect();
        Ok(Table::new(items?).with_header(vec!["ID", "NAME", "TYPE", "CONDITION", "AMOUNT"]))
    }

    /// Consumes iterator of items and calculate basic statistics
    /// over the processed data.
    ///
    /// Currently, it calculates only a percentage of items of each
    /// condition.
    ///
    /// Amount of items to take and skip can be managed
    /// by [`Zomboid::set_take`] and [`Zomboid::set_skip`].
    pub fn describe(&mut self) -> Result<Table<Stat>, E> {
        let mut map_per_condition = HashMap::<String, u32>::new();
        let mut total = 0u32;

        for v in &mut self
            .it
            .by_ref()
            .skip(self._skip.unwrap_or(0))
            .take(self._take.unwrap_or(usize::MAX))
        {
            let item = v?;
            *map_per_condition.entry(item.condition).or_insert(0) += item.amount;
            total += item.amount;
        }

        let mut stats = Vec::<Stat>::with_capacity(map_per_condition.len());
        for (name, amount) in map_per_condition.into_iter() {
            stats.push(Stat {
                name,
                value: (amount as f64) / (total as f64),
            });
        }

        Ok(Table::new(stats)
            .with_header(vec!["CONDITION", "%"])
            .with_width(40))
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::Item, Zomboid};
    use csv::Writer;
    use std::error::Error;
    use std::path::Path;
    use std::path::PathBuf;
    use std::{fs, io};
    use uuid::Uuid;

    // TODO: use tempfile crate!
    const BASE_PATH: &str = "~/.cache/rust/testing";

    fn setup_csv() -> Result<PathBuf, Box<dyn Error>> {
        fs::create_dir_all(BASE_PATH)?;
        let path = format!("{}/{}.csv", BASE_PATH, Uuid::new_v4());
        let mut writer = Writer::from_path(&path)?;

        writer.write_record(["id", "name", "type", "condition", "amount"])?;
        writer.write_record(["1", "Hummer", "Tool", "Mint", "10"])?;
        writer.write_record(["2", "Nails", "Fasteners", "Good", "400"])?;
        writer.write_record(["2", "Nails", "Fasteners", "Mint", "100"])?;
        writer.write_record(["3", "Garden saw", "Tool", "New", "2"])?;
        writer.write_record(["4", "Metal saw", "Tool", "New", "2"])?;

        Ok(path.into())
    }

    fn teardown_csv(path: impl AsRef<Path>) -> io::Result<()> {
        fs::remove_file(path)?;
        Ok(())
    }

    #[ignore = "experiment"]
    #[test]
    fn csv_reader() {
        let file_path = setup_csv().unwrap();
        let mut r = csv::Reader::from_path(&file_path).unwrap();

        for res in r.deserialize() {
            let rec: Item = res.unwrap();
            println!("{:?}", rec);
        }

        teardown_csv(file_path).unwrap();
    }

    #[test]
    fn zomboid_csv() {
        let file_path = setup_csv().unwrap();
        let mut r = csv::Reader::from_path(&file_path).unwrap();
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

        teardown_csv(file_path).unwrap();
    }

    #[test]
    fn describe() {
        let file_path = setup_csv().unwrap();
        let mut r = csv::Reader::from_path(&file_path).unwrap();
        let mut z = Zomboid::new(r.deserialize());

        let table = z.describe().unwrap();
        println!("{}", table);
    }
}
