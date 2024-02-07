use crate::dice_event;
use std::sync::Arc;

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

pub fn int_to_digit_text(value : i16) -> &'static str {
    match value {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        _ => "None"
    }
}

pub fn parse_event_code(code: &str, die_face_num :&u8) -> Arc<dyn dice_event::DiceRollRequirement> {
    //parse single event
    let mut v : Vec<i16> = code.split(" or ").map(|name| digit_text_to_int(name))
        .filter(|value|  (0..=(i16::from(die_face_num.clone()))).contains(value)) 
    .collect();
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();

    Arc::new(dice_event::SingleRollMultipleValueRequirement{possible_values : v, die_faces: die_face_num.clone()})
}
