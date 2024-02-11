use conv::*;
use std::collections::HashMap;
use std::sync::Arc;
use malachite::Natural;
use malachite::Rational;
use malachite::num::arithmetic::traits::Factorial;
use malachite::num::arithmetic::traits::Pow;
//use malachite::num::float::NiceFloat;
//use malachite::num::basic::floats::PrimitiveFloat;
//use malachite::num::basic::traits::OneHalf;
use malachite::rounding_modes::RoundingMode;
use malachite::num::conversion::traits::RoundingFrom;
//use malachite::Integer;

pub trait DiceRollRequirement {
    fn success_probability_for_one_turn(&self) -> f64;
    fn expected_turns(&self) -> f64;
    fn enumerate_roll_values(&self) -> Vec<i16>;
}

pub trait FulfillableRequirement : DiceRollRequirement {
    fn fullfill_with(&mut self, single_roll : &i16) -> bool;
}


pub struct InvalidRequirement {
}

impl DiceRollRequirement for InvalidRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        0.0_f64
    }

    fn expected_turns(&self) -> f64 {
        1.0_f64
    }

    fn enumerate_roll_values(&self) -> Vec<i16> {
        Vec::<i16>::new()
    }
}

impl FulfillableRequirement for InvalidRequirement {
    fn fullfill_with(&mut self, _single_roll : &i16) -> bool {
        false
    }
}

pub struct SingleValueRequirement {
    pub required_value : i16,
    pub die_faces : u8,
}

impl SingleValueRequirement {
    pub fn new(val : i16) -> Self {
        SingleValueRequirement{required_value:val, die_faces: 6}
    }
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
}

impl FulfillableRequirement for SingleValueRequirement {
    fn fullfill_with(&mut self, single_roll : &i16) -> bool {
        single_roll == &self.required_value
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
}

impl FulfillableRequirement for SingleRollMultipleValueRequirement {
    fn fullfill_with(&mut self, single_roll : &i16) -> bool {
        self.possible_values.contains(&single_roll)
    }
}

pub struct SuccessiveRollsInMultipleTurnsRequirement {
    pub rolls : Vec<i16>,
    pub die_faces : u8,
}

impl DiceRollRequirement for SuccessiveRollsInMultipleTurnsRequirement  {
    fn success_probability_for_one_turn(&self) -> f64 {
        let mut value_partitions = HashMap::<i16, u32>::new();
        for roll in &self.rolls {
            *value_partitions.entry(roll.clone()).or_insert(0) += 1;
        }

        
        // Combinatorial factorial division : 21!/(6! * 5! * 4! * 3! * 2! * 1!)
        let factorial_num = Natural::factorial(u64::value_from(self.rolls.len()).expect("OH NO"));
        let factorial_denom_it = value_partitions.iter().map(|pair | Natural::factorial(u64::from(pair.1.clone())));  
        let factorial_denom : Natural = factorial_denom_it.product();

        let factorial_quotient: Natural = factorial_num / factorial_denom;
        
        // (1/6)^21
        let proba_denom : Natural = Natural::from(6u32).pow(u64::value_from(self.rolls.len()).expect("OH NO"));
        let proba_quotient : Rational = Rational::from_naturals(Natural::from(1u32), proba_denom);

        let rational_result = Rational::from(factorial_quotient) * proba_quotient;

        f64::rounding_from(rational_result, RoundingMode::Floor).0
    }
    fn expected_turns(&self) -> f64 {
        1.0_f64 / self.success_probability_for_one_turn() //due to accumulation of previous values this is way more complicated than that
    }
    fn enumerate_roll_values(&self) -> Vec<i16> {
        //let mut all_values = self.rolls.iter().map(|roll| roll.enumerate_roll_values()).fold(Vec::<i16>::new(), |mut accum, vec| {accum.extend(vec); accum}); //that compiled too btw
        let mut vec = self.rolls.clone();
        vec.sort();
        vec.dedup();
        vec
    }

}

impl FulfillableRequirement for SuccessiveRollsInMultipleTurnsRequirement {
    fn fullfill_with(&mut self, single_value : &i16) -> bool {
        
        let idx =  self.rolls.iter_mut().position(|roll| roll == single_value);
        if idx.is_some() {
         self.rolls.remove(idx.unwrap());
        }             
 
        idx.is_some() 
     }
}

pub struct ConsecutiveSequentialRollsRequirement {
    pub rolls : Vec<Arc<dyn FulfillableRequirement>>,
}

impl DiceRollRequirement for ConsecutiveSequentialRollsRequirement {
    fn success_probability_for_one_turn(&self) -> f64 {
        return self.rolls.iter().map(|roll|roll.success_probability_for_one_turn()).product()
    }

    fn expected_turns(&self) -> f64 {
        1.0_f64 / self.success_probability_for_one_turn()
    }

    fn enumerate_roll_values(&self) -> Vec<i16> {
        let mut all_values : Vec<i16> = self.rolls.iter().map(|roll| roll.enumerate_roll_values()).flatten().collect();
        
        all_values.sort();
        all_values.dedup();
        all_values
    }
}