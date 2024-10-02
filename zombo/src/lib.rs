struct Item {
    id: u32,
    name: String,

    // Add Enum for below's two fields
    item_type: String,
    condition: String,
    amount: u32,
}

struct Zomboid {
    data: Vec<Item>,
}

#[cfg(test)]
mod tests {}
