use conv::*;
use std::sync::Arc;

pub trait DiceRollRequirement {
    fn success_probability_for_one_turn(&self) -> f64;
    fn expected_turns(&self) -> f64;
    fn enumerate_roll_values(&self) -> Vec<i16>;
}

pub struct SingleDiceRollRequirement {
    pub possible_values : Vec<i16>,
    pub die_faces : u8,
}

impl DiceRollRequirement for SingleDiceRollRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        f64::value_from(self.possible_values.len()).expect("OH NO") / f64::from(self.die_faces.clone())
    }
    fn expected_turns(&self) -> f64 {
        1.0_f64 / self.success_probability_for_one_turn()
    }
    fn enumerate_roll_values(&self) -> Vec<i16> {
        self.possible_values.clone()
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
    pub rolls : Vec<Arc<dyn DiceRollRequirement>>,
    pub sequential : Sequential,
    pub consecutive : Consecutive,

}

impl DiceRollRequirement for SuccessiveDiceRollsRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        //MOAR POLYMORPHISM COULD BE POSSIBLE
        if self.sequential == Sequential::Yes && self.consecutive == Consecutive::Yes {
            return self.rolls.iter().map(|roll|roll.success_probability_for_one_turn()).fold(1.0_f64, |prod, x| prod * x);
        }
        0.0_f64
    }
    fn expected_turns(&self) -> f64 {
        1000.0_f64
    }
    fn enumerate_roll_values(&self) -> Vec<i16> {
        //let mut all_values = self.rolls.iter().map(|roll| roll.enumerate_roll_values()).fold(Vec::<i16>::new(), |mut accum, vec| {accum.extend(vec); accum}); //that compiled too btw
        let mut all_values : Vec<i16> = self.rolls.iter().map(|roll| roll.enumerate_roll_values()).flatten().collect();
        
        all_values.sort();
        all_values.dedup();
        all_values
    }

}