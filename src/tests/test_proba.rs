#[cfg(test)]
mod test_proba {
    use crate::dice_event_parser;
    use crate::dice_event;
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
}