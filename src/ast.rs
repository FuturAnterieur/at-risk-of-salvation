pub enum DiceRollExpr {
    SingleValue(i16),
    OrExpr(Box<DiceRollExpr>, Box<DiceRollExpr>),
}