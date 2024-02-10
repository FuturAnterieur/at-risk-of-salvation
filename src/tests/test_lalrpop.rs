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

    let parsed_expr = dice::DiceRollExprParser::new().parse("One or 6 or 2");
    assert!(parsed_expr.is_ok());
    let result = parsed_expr.ok().unwrap();
    let rolls = dice_event_parser::parse_dice_roll_expr_ast(result.as_ref(), &6).as_ref().enumerate_roll_values();
    assert_eq!(rolls, vec![1,2,6]);
}