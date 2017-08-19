use num::bigint::BigUint;

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    _Unknown,
    Whitespace,

    // == Literals == //
    IntegerLiteral(BigUint),
    StringLiteral(String),
}
