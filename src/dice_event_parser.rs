use crate::dice_event;
use std::sync::Arc;
use crate::ast::DiceRollExpr;

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

pub fn parse_dice_roll_expr_ast(code: &DiceRollExpr, num_faces : &u8) -> Arc<dyn dice_event::DiceRollRequirement> {
    match &code {
        DiceRollExpr::SingleValue(val) => Arc::new(dice_event::SingleValueRequirement{required_value: val.clone(), die_faces: num_faces.clone()}),
        DiceRollExpr::OrExpr(left, right ) => {
            let mut left_parse = sub_parse_dice_roll_or_expr(left);
            let mut right_parse = sub_parse_dice_roll_or_expr(right);
            left_parse.extend(right_parse.iter());
            left_parse.sort();
            left_parse.dedup();

            return Arc::new(dice_event::SingleRollMultipleValueRequirement{possible_values: left_parse, die_faces : num_faces.clone()})
        }
            
    }
}

pub fn sub_parse_dice_roll_or_expr(code: &DiceRollExpr) -> Vec<i16> {
    match &code {
        DiceRollExpr::SingleValue(val) => vec![val.clone()],
        DiceRollExpr::OrExpr(left, right) => {
            let mut left_parse = sub_parse_dice_roll_or_expr(left);
            let right_parse = sub_parse_dice_roll_or_expr(right);
            left_parse.extend(right_parse.iter());
            return left_parse
        }
    }
}