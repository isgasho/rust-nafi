//! Parsing result types

/// Standard result type for parsing.
pub type Result<In, Out, Error> = ::std::result::Result<ParseOutput<In, Out>, Error>;

/// The output of a parser fragment.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ParseOutput<In, Out> {
    /// Unused input given to the parser that produced this output
    pub remaining_input: In,
    /// The transformed output from the parser that produced this output
    pub output: Out,
}
