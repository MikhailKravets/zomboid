use crate::model::Item;
use std::fmt::Display;

// TODO: data should be also a generic over T: Display
#[derive(Debug)]
pub struct Table<H> {
    header: Option<Vec<H>>,
    data: Vec<Item>,
}

impl<H: Display> Table<H> {
    pub fn new(data: Vec<Item>) -> Self {
        Self { header: None, data }
    }

    pub fn with_header(mut self, header: Vec<H>) -> Self {
        self.header = Some(header);
        self
    }
}

impl<H> Table<H> {
    pub fn as_data(&self) -> &Vec<Item> {
        &self.data
    }
}

impl<H: Display> Display for Table<H> {
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
    use super::*;

    #[test]
    fn table_header() {
        let item = Item {
            id: 1,
            name: "Test".into(),
            item_type: "Test".into(),
            condition: "Good".into(),
            amount: 10,
        };
        let table = Table::<String>::new(vec![item]);

        assert_eq!(table.data.len(), 1);
        assert_eq!(table.data[0].id, 1);

        println!("{}", table);
    }
}
