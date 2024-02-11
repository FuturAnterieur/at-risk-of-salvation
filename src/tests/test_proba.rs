#[cfg(test)]
mod test_proba {
    use crate::dice_event_parser;
    use crate::dice_event;
    use crate::dice_event::DiceRollRequirement;
    use assert_float_eq::*;
    

    #[test]
    fn test_proba_from_code() {
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("One", &6).success_probability_for_one_turn(), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("One or One", &6).success_probability_for_one_turn(), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("Two or Four or Five", &6).success_probability_for_one_turn(), (0.5_f64));
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("21 or Four or 5", &6).success_probability_for_one_turn(), 1.0_f64/3.0_f64);
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("1000 or Thirty-Three", &6).success_probability_for_one_turn(), 0.0_f64);
        assert_float_absolute_eq!(dice_event_parser::parse_dice_roll_expr_ast("bogus stuff", &6).success_probability_for_one_turn(), 0.0_f64);
    }

    #[test]
    fn test_vajra_hell_dice_event() {
        let mut vajra_one_turn = dice_event::SuccessiveRollsInMultipleTurnsRequirement{rolls: Vec::<i16>::new(), die_faces: 6};
        vajra_one_turn.rolls.push(1);
        
        vajra_one_turn.rolls.push(2);
        vajra_one_turn.rolls.push(2);
        
        vajra_one_turn.rolls.push(3);
        vajra_one_turn.rolls.push(3);
        vajra_one_turn.rolls.push(3);
        
        vajra_one_turn.rolls.push(4);
        vajra_one_turn.rolls.push(4);
        vajra_one_turn.rolls.push(4);
        vajra_one_turn.rolls.push(4);
        
        vajra_one_turn.rolls.push(5);
        vajra_one_turn.rolls.push(5);
        vajra_one_turn.rolls.push(5);
        vajra_one_turn.rolls.push(5);
        vajra_one_turn.rolls.push(5);

        vajra_one_turn.rolls.push(6);
        vajra_one_turn.rolls.push(6);
        vajra_one_turn.rolls.push(6);
        vajra_one_turn.rolls.push(6);
        vajra_one_turn.rolls.push(6);
        vajra_one_turn.rolls.push(6);

        assert_float_absolute_eq!(vajra_one_turn.success_probability_for_one_turn(), 0.0000935961_f64)
    }

    #[test]
    fn vajra_from_code() {
        let event = dice_event_parser::parse_dice_roll_expr_ast("accum: 1 + 2 x 2 + 3 x 3 + 4 x 4 + 5 x 5 + 6 x 6", &6);
        let proba =event.success_probability_for_one_turn(); 
        assert_float_absolute_eq!(proba, 0.0000935961_f64);

        //With accum, different order should not change anything
        let event_2 = dice_event_parser::parse_dice_roll_expr_ast("accum: 2 x 2 + 4 x 4 + 3 x 3 + One + 6 x 6 + 5 x 5", &6);
        assert_float_absolute_eq!(event_2.success_probability_for_one_turn(),proba);
    }

}