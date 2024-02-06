use conv::*;
use crate::dice_event;

pub fn digit_text_to_int(text : &str) -> i16 {
    match text {
        "One" => 1,
        "1" => 1,
        "Two" => 2,
        "2" => 2,
        "Three" => 3,
        "3" => 3,
        "Four" => 4,
        "4" => 4,
        "Five" => 5,
        "5" => 5,
        "Six" => 6,
        "6" => 6,
        _ => -1,
    }
}


pub fn get_proba_from_code(code: &str, die_face_num :&u8) -> f64 {
    let mut v : Vec<i16> = code.split(" or ").map(|name| digit_text_to_int(name))
        .filter(|value|  (0..=(i16::from(die_face_num.clone()))).contains(value)) 
    .collect();
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();

    f64::value_from(v.len()).expect("OH NO") / f64::from(die_face_num.clone())
}

pub fn get_expected_rolls_from_code(code : &str, die_face_num :&u8) -> f64 {
    //simple cases (not Vajra hell) :
    1.0_f64 / get_proba_from_code(code, die_face_num)
}