use location::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Identifier<'a> {
    pub text: &'a str,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Operator<'a> {
    pub op: &'a str,
    pub span: Span,
}
