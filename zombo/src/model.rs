#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,

    // Add Enum for below's two fields
    #[serde(rename = "type")]
    pub item_type: String,
    pub condition: String,
    pub amount: u32,
}
