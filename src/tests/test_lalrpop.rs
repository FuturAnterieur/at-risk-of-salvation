use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub dice);

#[test]
fn dice() {
    assert!(dice::DiceRollExprParser::new().parse("Three or Two").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("3 or Two").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("Three or 2").is_ok());
    assert!(dice::DiceRollExprParser::new().parse("1").is_ok());



}