use std::fmt::Display;

pub trait RowDisplay {
    fn to_row(&self, table_width: usize) -> String;
}

#[derive(Debug)]
pub struct Table<T> {
    header: Option<Vec<&'static str>>,
    width: usize,
    data: Vec<T>,
}

impl<T> Table<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            header: None,
            data,
            width: 100,
        }
    }

    pub fn with_header(mut self, header: Vec<&'static str>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn top_sep(&self) -> String {
        let width = self.width - 2;
        format!("┌{:─^width$}┐", "")
    }

    pub fn middle_sep(&self) -> String {
        let width = self.width - 2;
        format!("├{:─^width$}┤", "")
    }

    pub fn bottom_sep(&self) -> String {
        let width = self.width - 2;
        format!("└{:─^width$}┘", "")
    }
}

impl<T> Table<T> {
    pub fn as_data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<H: Display> RowDisplay for Vec<H> {
    fn to_row(&self, table_width: usize) -> String {
        let width = table_width / self.len() - 3;
        let mut s = String::new();
        for v in self {
            s.push_str(&format!("│ {:^width$} ", v));
        }

        // Last column will always have 1 redundant char at the end.
        s.pop();
        s.push('│');

        s
    }
}

impl<T: RowDisplay> Display for Table<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = self.top_sep();
        let mid = self.middle_sep();
        let bot = self.bottom_sep();

        writeln!(f, "{}", top)?;
        if let Some(header) = &self.header {
            writeln!(f, "{}", header.to_row(self.width))?;
            writeln!(f, "{}", mid)?;
        }

        for v in &self.data {
            writeln!(f, "{}", v.to_row(self.width))?;
        }

        writeln!(f, "{}", bot)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Item;

    #[test]
    fn table_with_header() {
        let item = Item {
            id: 1,
            name: "Test".into(),
            item_type: "Test".into(),
            condition: "Good".into(),
            amount: 10,
        };
        let table =
            Table::new(vec![item]).with_header(vec!["ID", "NAME", "TYPE", "CONDITION", "AMOUNT"]);
    }

    #[test]
    fn table_without_header() {
        let item = Item {
            id: 1,
            name: "Test".into(),
            item_type: "Test".into(),
            condition: "Good".into(),
            amount: 10,
        };
        let table = Table::new(vec![item]);

        assert_eq!(table.data.len(), 1);
        assert_eq!(table.data[0].id, 1);

        println!("{}", table);
    }
}
