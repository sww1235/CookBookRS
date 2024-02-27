use super::step::Step;

/// `Recipe` represents one recipe from start to finish
pub struct Recipe {
    /// database ID
    pub id: u64,
    /// short name of recipe
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// recipe comments
    pub comments: Option<String>,
    /// recipe source
    pub source: String,
    /// recipe author
    pub author: String,
    //pub amount_made: TODO: units,
    /// list of steps in recipe
    pub steps: Vec<Step>,
    //TODO: tags, versions
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
}
//TODO: function to return total amount of time, ingredients, and equipment needed
