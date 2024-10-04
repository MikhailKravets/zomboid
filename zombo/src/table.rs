use crate::model::Item;
use std::fmt::Display;

#[derive(Debug)]
pub struct Table {
    header: Option<Vec<&'static str>>,
    data: Vec<Item>,
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
