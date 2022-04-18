use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Define structs to explain config.yml structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct App {
    url: String,
    display_url: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Bookmark {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    apps: HashMap<String, App>,
    bookmarks: HashMap<String, Vec<Bookmark>> 
}

impl Config {
    pub fn new() -> Config { 
        Config {apps: HashMap::new(), bookmarks: HashMap::new()}
    }
}