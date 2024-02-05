use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Square {

    pub number: u32,
    pub name:String,
    pub description:String,
    pub paths:HashMap<String,u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub die_faces: u8,
    pub winning_square:u32,
    pub starting_square:u32,
    pub squares:Vec<Square>,
}

pub fn build_game_from_json_string(s : &str) -> Result<Game> {

    serde_json::from_str(s)
}
