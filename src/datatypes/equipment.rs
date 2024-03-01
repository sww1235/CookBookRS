/// `Equipment` represents any implement you might use to prepare a recipe,
/// from a stove, to a microwave, to a stand mixer, to a potato peeler
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Equipment {
    /// database unique ID
    pub id: u64,
    /// short name of item
    pub name: String,
    /// longer description of item
    pub description: Option<String>,
    /// If item is owned. Allows filtering out recipes that require equipment you don't own so you
    /// don't get half way through a recipe and realize it needs some specialized piece of
    /// equipment like a melon baller or pineapple corer
    pub is_owned: bool,
}
