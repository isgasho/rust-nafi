#[macro_use]
extern crate pretty_assertions;
extern crate sexpr;
use sexpr::SExpr;

#[test]
fn if_else_expr() {
    let tree = r#"
(SideEffect
 (FunctionCall
  (Identifier "if")
  (Whitespace " ")
  (Symbol "(")
  (Identifier "true")
  (Symbol ")")
  (Whitespace " ")
  (Closure
   (Symbol "{")
   (Whitespace "\n    ")
   (SideEffect
    (FunctionCall
     (Identifier "print")
     (Symbol "(")
     (FunctionCallArgument
      (StringLiteral
       (Symbol "\"")
       (StringText "true is true")
       (Symbol "\"")))
     (Symbol ")"))
    (Symbol ";"))
   (Whitespace "\n")
   (Symbol "}")))
 (Symbol ";"))
    "#.trim();
    let expr: SExpr = tree.parse().unwrap();
    assert_eq!(tree, expr.multi_line().to_string())
}
