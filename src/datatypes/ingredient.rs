/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Debug, Clone, Default)]
pub struct Ingredient {
    /// database ID
    pub id: u64,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    //TODO: units
    //TODO: inventory reference
    //TODO: sum all ingredients into master list of ingredients for top of recipe
}
