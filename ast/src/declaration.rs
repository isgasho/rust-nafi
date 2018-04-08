use location::Span;

use statement::Assignment;

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Declaration<'a> {
    Binding(#[serde(borrow)] Box<Binding<'a>>),
}

impl<'a> Declaration<'a> {
    pub fn span(&self) -> Span {
        match self {
            Declaration::Binding(decl) => decl.span,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Binding<'a> {
    pub mutable: bool,
    #[serde(borrow)]
    pub assignment: Assignment<'a>,
    pub span: Span,
}
