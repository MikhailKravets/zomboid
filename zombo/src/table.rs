//! Table module provides simple terminal table formatter.
//! [`Table`] struct implements [`Display`] trait that formats data to the table similar to
//!
//! ```ignore
//! ┌─────────────────────────────┐
//! │   Header 1   │   Header 2   │
//! ├─────────────────────────────┤
//! │     One      │     Two      │
//! │    Three     │     Four     │
//! └─────────────────────────────┘
//! ```
//!
//! Struct [`Table`] is generic over cell type `T`. Ensure that type `T` implements
//! [`RowDisplay`] trait. The method [`RowDisplay::to_row`] returns a [`String`] that
//! represent formatted table row.
//!
//! # Examples
//!
//! Imagine we have an `Row` struct then the implementation of [`RowDisplay`] for `Row`
//! and further usage of [`Table`] struct could be
//!
//! ```rust
//! use zombo::table::RowDisplay;
//! use zombo::table::Table;
//!
//! struct Row {
//!     id: usize,
//!     name: String
//! }
//!
//! impl RowDisplay for Row {
//!     fn to_row(&self, table_width: usize) -> String {
//!         // table_width is the width of table in characters.
//!         //
//!         // Divide on 2 because Row has two fields and we
//!         // want to give them both cells an equal width.
//!         //
//!         // Minus 3 because we add 3 additional chars to each cell
//!         let width = table_width / 2 - 3;
//!         format!("│ {:^width$} │ {:^width$}│", self.id, self.name)
//!     }
//! }
//!
//! let data = vec![
//!     Row {id: 1, name: "One".into()},
//!     Row {id: 2, name: "Two".into()}
//! ];
//! let table = Table::new(data)
//!                 .with_header(vec!["COL1", "COL2"])
//!                 .with_width(90);
//! println!("{table}");
//!
//! // Borrow table data immutably
//! let data = table.as_data();
//! ```
//!
//! Currently [`Table`] only supports header of static strings. However, this is a subject
//! to change later.
use std::fmt::Display;

/// A trait to implement if you want a type to be formatted
/// as a row of a table.
///
/// You might use this symbol `│`.
pub trait RowDisplay {
    /// # Arguments
    ///
    /// * `table_width` is a table width in characters. This argument may be useful to
    ///                 calculate the size of a cell of a row.
    fn to_row(&self, table_width: usize) -> String;
}

/// Table represents a container for data to be formatted as a table.
/// Optionally, you may set a header to the table and width in characters.
/// Currenty, table header accepts only a Vec of 'static strings.
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

    fn top_sep(&self) -> String {
        let width = self.width - 2;
        format!("┌{:─^width$}┐", "")
    }

    fn middle_sep(&self) -> String {
        let width = self.width - 2;
        format!("├{:─^width$}┤", "")
    }

    fn bottom_sep(&self) -> String {
        let width = self.width - 2;
        format!("└{:─^width$}┘", "")
    }
}

impl<T> Table<T> {
    pub fn as_data(&self) -> &Vec<T> {
        &self.data
    }
}

/// This is an implementation of RowDisplay for table header.
/// Potentially header can be something bigger then just `&'static str`,
/// so this implementation is generic.
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

        write!(f, "{}", bot)?;
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
        let header = vec!["ID", "NAME", "TYPE", "CONDITION", "AMOUNT"];
        let items = vec![item];

        let table = Table::new(items).with_header(header.clone());
        let table_string = format!("{}", table);
        let rows: Vec<&str> = table_string.split("\n").collect();

        // 1 - header
        // 1 - item
        // 2 - top / bottom separators
        // 1 - bottom separator
        assert_eq!(rows.len(), 1 + 1 + 2 + 1);
        for v in header.iter() {
            assert!(rows[1].contains(v));
        }

        assert!(rows[3].contains(&format!("{}", table.data[0].id)));
        assert!(rows[3].contains(&table.data[0].name.to_string()));
        assert!(rows[3].contains(&table.data[0].item_type.to_string()));
        assert!(rows[3].contains(&table.data[0].condition.to_string()));
        assert!(rows[3].contains(&format!("{}", table.data[0].amount)));

        println!("{}", table_string);
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

        let table_string = format!("{}", table);
        let rows: Vec<&str> = table_string.split("\n").collect();

        // 1 - item
        // 2 - top / bottom
        assert_eq!(rows.len(), 2 + 1);
        println!("{}", table);
    }
}
