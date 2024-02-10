//use lalrpop_util::lalrpop_mod;

//lalrpop_mod!(pub dice);

use crate::lalrpop::dice;
use crate::dice_event_parser;

#[test]
fn dice() {
    assert!(dice::DiceRollExprParser::new().parse("Three or Two").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("3 or Two").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("Three or 2").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("1").is_ok());

    let rolls = dice_event_parser::parse_dice_roll_expr_ast("One or 6 or 2", &6).enumerate_roll_values();
    assert_eq!(rolls, vec![1,2,6]);

    let rolls2 = dice_event_parser::parse_dice_roll_expr_ast("1 to 3 or 5", &6).enumerate_roll_values();
    assert_eq!(rolls2, vec![1,2,3,5]);

    let rolls3 = dice_event_parser::parse_dice_roll_expr_ast("1 or 3 to 7", &20).enumerate_roll_values();
    assert_eq!(rolls3, vec![1,3,4,5,6,7]);

    let rolls4 = dice_event_parser::parse_dice_roll_expr_ast("1 or 3 to 7 or 10 to 12", &20).enumerate_roll_values();
    assert_eq!(rolls4, vec![1,3,4,5,6,7,10,11,12]);
}