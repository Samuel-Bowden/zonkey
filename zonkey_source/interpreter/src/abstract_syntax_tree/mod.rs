use crate::{literal::Literal, token::token_type::TokenType};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Literal(Literal),
}

#[derive(Debug)]
pub struct AbstractSyntaxTree(pub Expr);

#[cfg(test)]
mod tests {
    use crate::token::token_type::TokenType;
    use super::*;

    #[test]
    fn construct_and_print() {
        let ast = AbstractSyntaxTree(
            Expr::Binary { 
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(Literal::Integer(1))),
                    operator: TokenType::Minus,
                    right: Box::new(Expr::Literal(Literal::Integer(2))),
                }),
                operator: TokenType::Minus,
                right: Box::new(Expr::Literal(Literal::Integer(3))),
            }
        );

        println!("{:?}", ast);
    }
}
