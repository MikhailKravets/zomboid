use model::Item;
use std::fmt::Display;

pub mod model;

#[derive(Debug)]
pub struct Zomboid<T> {
    it: T,
}

#[derive(Debug)]
pub struct Table {
    header: Option<Vec<&'static str>>,
    data: Vec<Item>,
}

impl<T, E> Zomboid<T>
where
    T: Iterator<Item = Result<Item, E>>,
    E: std::error::Error,
{
    pub fn new(it: T) -> Self {
        Self { it }
    }

    pub fn stream(&mut self, take: usize, skip: usize) -> Result<Table, E> {
        let it = &mut self.it;
        let items: Result<Vec<Item>, E> = it.skip(skip).take(take).collect();

        Ok(Table::new(items?).with_header(vec!["ID", "NAME", "TYPE", "CONDITION", "AMOUNT"]))
    }
}

impl Table {
    pub fn new(data: Vec<Item>) -> Self {
        Self { header: None, data }
    }

    pub fn with_header(mut self, header: Vec<&'static str>) -> Self {
        self.header = Some(header);
        self
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = format!("┌{:─^114}┐", "");
        let bot = format!("└{:─^114}┘", "");
        let mid = format!("├{:─^114}┤", "");

        writeln!(f, "{}", top)?;
        if let Some(header) = &self.header {
            for v in header {
                write!(f, "│ {:^20} ", v)?;
            }

            write!(f, "│")?;
            writeln!(f)?;

            writeln!(f, "{}", mid)?;
        }

        for item in &self.data {
            writeln!(
                f,
                "│ {:^20} │ {:^20} │ {:^20} │ {:^20} │ {:^20} │",
                item.id, item.name, item.item_type, item.condition, item.amount
            )?;
        }
        writeln!(f, "{}", bot)?;
        Ok(())
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
        println!("{}", z.stream(10, 0).unwrap())
    }
}
