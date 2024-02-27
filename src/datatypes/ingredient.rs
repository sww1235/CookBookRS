/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Debug, Clone, Default)]
pub struct Ingredient {
    id: u64,
    name: String,
    description: Option<String>,
    //TODO: units
    //TODO: step reference
    //TODO: inventory reference
    //TODO: sum all ingredients into master list of ingredients for top of recipe
}
