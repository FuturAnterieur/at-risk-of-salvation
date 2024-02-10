
pub enum DiceRollExpr {
    SingleValue(i16),
    OrExpr(Box<DiceRollExpr>, Box<DiceRollExpr>),
    ToExpr(i16, i16),
}
