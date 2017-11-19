use std::result::Result;

/// The result of a parser fragment.
pub type ParseResult<In, Out, Error> = Result<ParseOutput<In, Out>, Error>;

/// The output of a parser fragment.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ParseOutput<In, Out> {
    /// Unused input given to the parser that produced this output
    pub remaining_input: In,
    /// The transformed output from the parser that produced this output
    pub output: Out,
}
