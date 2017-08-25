use std::fmt;

/// The output of a parser combinator piece.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Result<In, Out = In, Failure = String>
where
    Failure: fmt::Display,
{
    /// Parsing succeeded.
    Done(In, Out),

    /// Parsing failed, and should terminate immediately with a message.
    Abort(Failure),

    /// Parsing failed, and may be able to recover.
    Pass,
}
