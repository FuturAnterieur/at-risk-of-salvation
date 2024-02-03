use serde::{Deserialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Square {

    pub number: u32,
    pub name:String,
    pub description:String,
    pub paths:HashMap<String,u32>,
}

#[derive(Deserialize)]
pub struct Game {
    pub die_faces: u8,
    pub squares:Vec<Square>,
}

pub fn build_game_from_json_string(s : &str) -> Result<Game> {

    serde_json::from_str(s)
}