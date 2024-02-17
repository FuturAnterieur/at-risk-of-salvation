use crate::dice_event;
use serde::{Deserialize, Serialize};
use crate::graph;
use dyn_clone::clone_box;
pub struct RemainingRequirementsForEdge {
    pub remaining : Box<dyn dice_event::FulfillableRequirement>,
}

#[derive(Deserialize, Serialize)]
pub struct PlayerStatusPersistentData {
    pub name : String,
    pub current_square : u32,
    pub rolls_on_current_square : Vec<i16>,
}

pub struct PlayerStatus {
    pub data : PlayerStatusPersistentData,
    pub remaining_reqs_for_each_edge : Vec<RemainingRequirementsForEdge>,
}

impl PlayerStatus {
    pub fn new(name : &str, starting_square:u32) -> Self {
        PlayerStatus{data:PlayerStatusPersistentData{name:name.to_string(), current_square:starting_square, rolls_on_current_square: Vec::<i16>::new()}, remaining_reqs_for_each_edge: Vec::<RemainingRequirementsForEdge>::new()}
    }

    pub fn from_persistent_data(psd : PlayerStatusPersistentData, g : &graph::Graph) -> Self {
        let ps = PlayerStatus{data:psd, remaining_reqs_for_each_edge : Vec::<RemainingRequirementsForEdge>::new()};
        ps
    }
}
