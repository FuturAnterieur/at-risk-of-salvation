use conv::*;

pub trait DiceRollRequirement {
    fn success_probability(&self) -> f64;
    fn expected_attempts(&self) -> f64;
}

pub struct SingleDiceRollRequirement {
    pub possible_values : Vec<i16>,
    pub die_faces : u8,
}

impl DiceRollRequirement for SingleDiceRollRequirement {
    fn success_probability(&self) -> f64 {
        f64::value_from(self.possible_values.len()).expect("OH NO") / f64::from(self.die_faces.clone())
    }
    fn expected_attempts(&self) -> f64 {
        1.0_f64 / self.success_probability()
    }
}

            
#[derive(PartialEq)]
pub enum Sequential {
    No,
    Yes,
}
#[derive(PartialEq)]
pub enum Consecutive {
    No,
    Yes,
}

pub struct SuccessiveDiceRollsRequirement {
    pub rolls : Vec<Box<dyn DiceRollRequirement>>,
    pub sequential : Sequential,
    pub consecutive : Consecutive,

}

impl DiceRollRequirement for SuccessiveDiceRollsRequirement {
    fn success_probability(&self) -> f64 {
        //MOAR POLYMORPHISM COULD BE POSSIBLE
        if self.sequential == Sequential::Yes && self.consecutive == Consecutive::Yes {
            return self.rolls.iter().map(|roll|roll.success_probability()).fold(1.0_f64, |prod, x| prod * x);
        }
        0.0_f64
    }
    fn expected_attempts(&self) -> f64 {
        1000.0_f64
    }

}