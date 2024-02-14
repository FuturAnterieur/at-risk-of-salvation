use crate::dice_event;

pub struct RemainingRequirementsForEdge {
    pub remaining : Box<dyn dice_event::FulfillableRequirement>,
}

pub struct PlayerStatus {
    pub name : String,
    pub rolls_on_current_square : Vec<i16>,
    pub remaining_reqs_for_each_edge : Vec<RemainingRequirementsForEdge>,
}
