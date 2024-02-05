#[cfg(test)]
mod tests {
    use crate::probability_parser;
    use assert_float_eq::*;
    

    #[test]
    fn test_proba_from_code() {
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("One", &6), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("One or One", &6), (1.0_f64/6.0_f64));
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("Two or Four or Five", &6), (0.5_f64));
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("Twenty-one or Four or 5", &6), 1.0_f64/3.0_f64);
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("1000 or Thirty-Three", &6), 0.0_f64);
        assert_float_absolute_eq!(probability_parser::get_proba_from_code("bogus stuff", &6), 0.0_f64);
    }
}