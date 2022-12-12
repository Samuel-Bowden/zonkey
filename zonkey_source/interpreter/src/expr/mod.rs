use crate::{literal::Literal, token::Token};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Literal),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_print() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(Literal::Integer(1))),
                operator: Token::Minus,
                right: Box::new(Expr::Literal(Literal::Integer(2))),
            }),
            operator: Token::Minus,
            right: Box::new(Expr::Literal(Literal::Integer(3))),
        };

        println!("{:?}", expression);
    }
}
