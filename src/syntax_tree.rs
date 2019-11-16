use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Value(i32),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

impl Node {
    fn evaluate(self) -> i32 {
        use self::Node::*;
        match self {
            Value(value) => value,
            Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Subtract(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            Multiply(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Divide(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
    root: Node,
}

impl SyntaxTree {
    pub fn from_tokens(tokens: Vec<Token>) -> Result<SyntaxTree, ParseError> {
        let tokens = tokens.into_iter();

        parse_expression(tokens).map(|root| SyntaxTree { root })
    }

    pub fn evaluate(self) -> i32 {
        self.root.evaluate()
    }
}

fn parse_expression(mut tokens: impl Iterator<Item = Token>) -> Result<Node, ParseError> {
    let mut value_queue: Vec<Node> = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Integer(digit) => value_queue.push(Node::Value(digit)),
            Token::Operator(operator) => {
                // on either side of operator there is an expression
                let inner_tokens = tokens.by_ref().take_while(|t| *t != Token::Rparen);
                match parse_expression(inner_tokens) {
                    Ok(rhs) => {
                        let lhs = value_queue.pop().unwrap();

                        let node = match operator {
                            '+' => Node::Add(Box::new(lhs), Box::new(rhs)),
                            '-' => Node::Subtract(Box::new(lhs), Box::new(rhs)),
                            '*' => Node::Multiply(Box::new(lhs), Box::new(rhs)),
                            '/' => Node::Divide(Box::new(lhs), Box::new(rhs)),
                            _ => panic!("unknown operator"),
                        };
                        value_queue.push(node);
                    }
                    Err(_) => {
                        return Err(ParseError::UnexpectedToken);
                    }
                }
            }
            Token::Lparen => {
                let inner_tokens: Vec<Token> = tokens
                    .by_ref()
                    .take_while(|token| *token != Token::Rparen)
                    .collect();
                assert_eq!(tokens.next(), Some(Token::Rparen));
                if let Ok(inner_expression) = SyntaxTree::from_tokens(inner_tokens) {
                    value_queue.push(inner_expression.root);
                }
            }
            Token::Rparen => return Err(ParseError::UnexpectedToken),
        }
    }

    let root = value_queue.pop().unwrap();

    // As we consume by operators, we know there's something wrong when there're
    // still values left when we're done with all of them
    if value_queue.len() > 0 {
        return Err(ParseError::TooManyValues);
    }

    Ok(root)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooManyValues,
    UnexpectedToken,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ast_two_plus_two() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(2),
                Token::Operator('+'),
                Token::Integer(2)
            ])
            .unwrap()
            .root,
            Node::Add(Box::new(Node::Value(2)), Box::new(Node::Value(2)))
        );
    }

    #[test]
    fn ast_two_plus_two_plus_two() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(2),
                Token::Operator('+'),
                Token::Integer(2),
                Token::Operator('+'),
                Token::Integer(2),
            ])
            .unwrap()
            .root,
            Node::Add(
                Box::new(Node::Add(
                    Box::new(Node::Value(2)),
                    Box::new(Node::Value(2)),
                )),
                Box::new(Node::Value(2)),
            )
        );
    }

    #[test]
    fn errors_for_too_much_values() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(2),
                Token::Operator('+'),
                Token::Integer(2),
                Token::Integer(2),
                Token::Integer(2),
                Token::Integer(2),
            ]),
            Err(ParseError::TooManyValues)
        );
    }

    #[test]
    fn errors_for_too_much_operators() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(2),
                Token::Operator('+'),
                Token::Operator('+'),
                Token::Integer(2),
                Token::Integer(2),
            ]),
            Err(ParseError::UnexpectedToken)
        );
    }

    #[test]
    fn test_left_associativity_operators() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(5),
                Token::Operator('-'),
                Token::Integer(3),
                Token::Operator('-'),
                Token::Integer(2),
            ]),
            Ok(SyntaxTree {
                root: Node::Subtract(
                    Box::new(Node::Subtract(
                        Box::new(Node::Value(5)),
                        Box::new(Node::Value(3))
                    )),
                    Box::new(Node::Value(2))
                )
            })
        )
    }

    #[test]
    fn test_paren_associativity() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(5),
                Token::Operator('+'),
                Token::Lparen,
                Token::Integer(3),
                Token::Operator('-'),
                Token::Integer(2),
                Token::Rparen,
            ])
            .unwrap()
            .evaluate(),
            6
        );
    }
}
