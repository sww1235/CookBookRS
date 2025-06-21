/// `root` contains the code for the root web page
pub mod root;

/// `browser` contains the code for the browser webpage
pub mod browser;

/// `recipe_editor` contains the code for the recipe editor and creator webpage
pub mod recipe_editor;

/// `error_responses` contains methods that return error responses
pub mod error_responses;

/// `media_responses` contains methods that return responses which contain media
/// such as favicon or image responses.
pub mod media_responses;

/// `html_stubs` contains common HTML components used across multiple web pages
pub mod html_stubs;

/// helper functions for various tasks when handling HTTP requests
pub mod http_helper;
