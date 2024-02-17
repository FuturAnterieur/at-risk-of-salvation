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
        let mut ps = PlayerStatus{data:psd, remaining_reqs_for_each_edge : Vec::<RemainingRequirementsForEdge>::new()};

        let original_edges = g.edges_for_node(&ps.data.current_square);

        /*if original_edges.is_none() {
            return Err(KarmicCatastrophe{message:"No edges found for node -- that's a programming error. Aborting.".to_string()});
        }*/

        for orig_edge in original_edges.unwrap_or(&Vec::<graph::Edge>::new()) {
            ps.remaining_reqs_for_each_edge.push(RemainingRequirementsForEdge{remaining: clone_box(&*orig_edge.requirement)});
        }

        for val in &ps.data.rolls_on_current_square {
            ps.remaining_reqs_for_each_edge.iter_mut().position(|reqs| reqs.remaining.fullfill_with(&val) );
        }
        
        ps
    }
}
