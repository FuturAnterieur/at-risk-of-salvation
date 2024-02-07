use crate::dice_event;
use std::sync::Arc;

pub struct RemainingRequirementsForEdge {
    pub remaining : Arc<dyn dice_event::DiceRollRequirement>,
}

pub struct PlayerStatus {
    pub name : String,
    pub rolls_on_current_square : Vec<i16>,
    pub remaining_reqs_for_each_edge : Vec<RemainingRequirementsForEdge>,
}
