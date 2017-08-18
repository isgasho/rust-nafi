#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    _Unknown,
    Whitespace,

    // == Literals == //
    IntegerLiteral(u64),
    StringLiteral(String),
}
