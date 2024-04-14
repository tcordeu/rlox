use crate::expr::Expr;

pub struct AstPrinter;

impl AstPrinter {
    pub fn pretty_print(e: &Expr) -> String {
        match *e {
            Expr::Literal(ref l) => match l {
                Some(v) => format!("{}", v),
                None => "None".to_string(),
            },
            Expr::Binary(ref lhs, ref token, ref rhs) => format!(
                "({} {} {})",
                token.lexeme(),
                Self::pretty_print(lhs),
                Self::pretty_print(rhs)
            ),
            Expr::Grouping(ref expr) => format!("(group {})", Self::pretty_print(expr)),
            Expr::Unary(ref token, ref expr) => {
                format!("({} {})", token.lexeme(), Self::pretty_print(expr))
            }
            Expr::Var(ref token) => format!("var {}", token.lexeme()),
            Expr::Assign(ref token, ref expr) => {
                format!("({} = {})", token.lexeme(), Self::pretty_print(expr))
            }
        }
    }
}
