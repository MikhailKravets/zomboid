use crate::table::RowDisplay;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,

    // TODO: Add Enum for below's two fields
    #[serde(rename = "type")]
    pub item_type: String,
    pub condition: String,
    pub amount: u32,
}

impl RowDisplay for Item {
    fn to_row(&self, table_width: usize) -> String {
        let width = table_width / 5 - 3;

        format!(
            "│ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$} │ {:^width$}│",
            self.id, self.name, self.item_type, self.condition, self.amount
        )
    }
}
