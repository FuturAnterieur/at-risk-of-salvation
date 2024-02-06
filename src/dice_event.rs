

pub trait DiceRollRequirement {
    fn success_probability(&self) -> f64;
    fn expected_attempts(&self) -> f64;
}

pub struct SingleDiceRollRequirement {
    pub possible_values : Vec<u8>,
    pub die_faces : u8,
}

enum Sequential {
    No,
    Yes,
}

enum Consecutive {
    No,
    Yes,
}

pub struct SuccessiveDiceRollsRequirement {
    pub rolls : Vec<Box<dyn DiceRollRequirement>>,
    pub sequential : Sequential,
    pub consecutive : Consecutive,

}