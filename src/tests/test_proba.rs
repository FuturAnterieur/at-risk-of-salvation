#[cfg(test)]
mod test_proba {
    use crate::dice_event_parser;
    use crate::dice_event;
    use crate::dice_event::DiceRollRequirement;
    use assert_float_eq::*;
    

    #[test]
    fn test_proba_from_code() {
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("One", &6).success_probability_for_one_turn(), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("One or One", &6).success_probability_for_one_turn(), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("Two or Four or Five", &6).success_probability_for_one_turn(), (0.5_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("Twenty-one or Four or 5", &6).success_probability_for_one_turn(), 1.0_f64/3.0_f64);
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("1000 or Thirty-Three", &6).success_probability_for_one_turn(), 0.0_f64);
        assert_float_absolute_eq!(dice_event_parser::parse_event_code("bogus stuff", &6).success_probability_for_one_turn(), 0.0_f64);
    }

    #[test]
    fn test_vajra_hell_dice_event() {
        let mut vajra_one_turn = dice_event::SuccessiveRollsInMultipleTurnsRequirement{rolls: Vec::<dice_event::SingleValueRequirement>::new()};
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(1));
        
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(2));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(2));
        
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(3));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(3));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(3));
        
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(4));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(4));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(4));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(4));
        
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(5));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(5));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(5));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(5));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(5));

        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));
        vajra_one_turn.rolls.push(dice_event::SingleValueRequirement::new(6));

        assert_float_absolute_eq!(vajra_one_turn.success_probability_for_one_turn(), 0.0000935961_f64)
    }
}