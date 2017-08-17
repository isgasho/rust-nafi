#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    _Unknown,
    _Whitespace,

    // == Literals == //
    IntegerLiteral(u64),
}
