
pub enum DiceRollExpr {
    SingleValue(i16),
    OrExpr(Box<DiceRollExpr>, Box<DiceRollExpr>),
    ToExpr(i16, i16),
}


pub enum SuccessiveRollsOptions{
    NotCumulative,
    Cumulative,
}

pub enum SuccessiveDiceRollExpr {
    SingleValue(i16),
    MulExpr(u32, i16),
    AndExpr(Box<SuccessiveDiceRollExpr>, Box<SuccessiveDiceRollExpr>),
}

pub enum AllDiceRollsExpr {
    NoRoll(()),
    SingleRoll(Box<DiceRollExpr>),
    SuccessiveRolls(SuccessiveRollsOptions, Box<SuccessiveDiceRollExpr>),
}