use conv::*;
use std::sync::Arc;
use std::ops::Range;

pub trait DiceRollRequirement {
    fn success_probability_for_one_turn(&self) -> f64;
    fn expected_turns(&self) -> f64;
    fn enumerate_roll_values(&self) -> Vec<i16>;
    fn fullfill_with(&mut self, single_roll : &i16) -> bool;
    fn num_dice_rolls_allowed(&self) -> usize;
}

pub struct SingleValueRequirement {
    pub required_value : i16,
    pub die_faces : u8,
}


impl DiceRollRequirement for SingleValueRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        1.0_f64 / f64::from(self.die_faces.clone())
    }

    fn expected_turns(&self) -> f64 {
        f64::from(self.die_faces.clone())
    }

    fn enumerate_roll_values(&self) -> Vec<i16> {
        vec![self.required_value.clone()]
    }

    fn fullfill_with(&mut self, single_roll : &i16) -> bool {
        single_roll == &self.required_value
    }

    fn num_dice_rolls_allowed(&self) -> usize {
        1
    }
}

pub struct SingleRollMultipleValueRequirement {
    pub possible_values : Vec<i16>,
    pub die_faces : u8,
}

impl DiceRollRequirement for SingleRollMultipleValueRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        f64::value_from(self.possible_values.len()).expect("OH NO") / f64::from(self.die_faces.clone())
    }
    fn expected_turns(&self) -> f64 {
        1.0_f64 / self.success_probability_for_one_turn()
    }
    fn enumerate_roll_values(&self) -> Vec<i16> {
        self.possible_values.clone()
    }

    fn fullfill_with(&mut self, single_value : &i16) -> bool {
        self.possible_values.contains(&single_value)
    }

    fn num_dice_rolls_allowed(&self) -> usize {
        1
    }
}



pub struct SuccessiveRollsInMultipleTurnsRequirement {
    pub rolls : Vec<SingleValueRequirement>,

}

impl DiceRollRequirement for SuccessiveRollsInMultipleTurnsRequirement  {
    fn success_probability_for_one_turn(&self) -> f64 {
        //MOAR POLYMORPHISM COULD BE POSSIBLE
        return self.rolls.iter().map(|roll|roll.success_probability_for_one_turn()).fold(1.0_f64, |prod, x| prod * x);
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

    fn fullfill_with(&mut self, sequence : &mut Vec<i16>, range_to_look_at : Range<usize>) -> bool {
        if self.consecutive == Consecutive::Yes && self.sequential == Sequential::Yes {
            let mut range_start:usize= range_to_look_at.start;
            let mut range_end:usize= range_to_look_at.start;
            self.rolls.retain(|roll| 
                    let num_to_be_consumed = roll.num_dice_rolls_allowed();
                    range_end += num_to_be_consumed;
                    let pre_length = sequence.len();
                    let it_worked = roll.fullfill_with(sequence, range_start..range_end);
                    let num_consumed = pre_length - sequence.len();
                    range_start += num_consumed; //hmmmm
                    range_end += num_consumed;
                
                !it_worked
            ); 
        }
        false
    }

    fn num_dice_rolls_allowed(&self) -> usize {
        self.rolls.iter().map(|roll| roll.num_dice_rolls_allowed()).sum()
    }

}