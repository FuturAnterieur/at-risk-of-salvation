use crate::dice_event;
use std::sync::Arc;
use crate::lalrpop::ast::{AllDiceRollsExpr, DiceRollExpr};
use crate::lalrpop;

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

pub fn parse_dice_roll_expr_ast(code_str: &str, num_faces : &u8) -> Arc<dyn dice_event::FulfillableRequirement> {
    let code = lalrpop::dice::AllDiceRollsExprParser::new().parse(&code_str);
    if code.is_err() {
        println!("Warning : could not parse the following dice event code : {}", code_str);
        return Arc::new(dice_event::InvalidRequirement{});
    }
    
    match code.expect("SHOULD NOT HAPPEN").as_ref() {
        AllDiceRollsExpr::SingleRoll(expr) => 
            match expr.as_ref() {
                DiceRollExpr::SingleValue(val) => Arc::new(dice_event::SingleValueRequirement{required_value: val.clone(), die_faces: num_faces.clone()}),
                DiceRollExpr::OrExpr(left, right ) => {
                    let mut left_parse = sub_parse_dice_roll_or_expr(left);
                    let right_parse = sub_parse_dice_roll_or_expr(right);
                    left_parse.extend(right_parse.iter());
                    let mut filtered : Vec<i16> = left_parse.into_iter().filter(|val| val >= &1 && val <= &i16::from(num_faces.clone())).collect();
                    filtered.sort();
                    filtered.dedup();

                    return Arc::new(dice_event::SingleRollMultipleValueRequirement{possible_values: filtered, die_faces : num_faces.clone()})
                },
                DiceRollExpr::ToExpr(beg, end) => {
            return Arc::new(dice_event::SingleRollMultipleValueRequirement{die_faces: num_faces.clone(),
                                possible_values:(beg.clone()..=end.clone()).collect() });}
            },
        AllDiceRollsExpr::SuccessiveRolls(_options, _expr) => Arc::new(dice_event::InvalidRequirement{})
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
        },
        DiceRollExpr::ToExpr(beg, end) => (beg.clone()..=end.clone()).collect()
    }
}