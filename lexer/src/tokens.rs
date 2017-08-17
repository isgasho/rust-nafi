#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    _Unknown,

    // == Literals == //
    IntegerLiteral(u64),
}
