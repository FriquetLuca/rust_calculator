// #[derive(Debug, PartialEq, Clone)]
// pub enum FunctionVariant {

// }

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    ExclamationMark,
    Modulo,
    LeftParen,
    RightParen,
    LeftFloor,
    RightFloor,
    LeftCeiling,
    RightCeiling,
    Pow2,
    Pow3,
    Pow,
    Sqrt,
    E,
    Pi,
    Abs,
    Floor,
    Ceil,
    Round,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Atan2,
    Arcosh,
    Arsinh,
    Artanh,
    Ln,
    Log,
    Exp,
    Exp2,
    Sign,
    Truncate,
    Comma,
    Min,
    Max,
    Bar,
    DegToRad,
    RadToDeg,
    Num(f64),
    Ans,
    Eof,
    //Function(),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
    Functional,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide | Modulo | DegToRad | RadToDeg => MulDiv,
            Caret | Pow2 | Pow3 => Power,
            ExclamationMark | Min | Max | Ln | Sign | Truncate | Log | Exp | Exp2 | Pow | Sqrt
            | Arcosh | Arsinh | Artanh | Abs | Floor | Ceil | Round | Sin | Cos | Tan | Sinh
            | Cosh | Tanh | Asin | Acos | Atan | Atan2 => Functional,
            _ => DefaultZero,
        }
    }
}
