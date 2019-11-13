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
        let mut value_queue: Vec<Node> = Vec::new();
        let mut tokens = tokens.into_iter();

        // Can't use `for_each` because when parsing operator, we lookahead for a next value
        while let Some(token) = tokens.next() {
            match token {
                Token::Integer(digit) => value_queue.push(Node::Value(digit)),
                Token::Operator(operator) => {
                    if let Some(Token::Integer(rhs)) = tokens.next() {
                        let rhs = Node::Value(rhs);
                        let lhs = value_queue.pop().unwrap();

                        let node = match operator {
                            '+' => Node::Add(Box::new(lhs), Box::new(rhs)),
                            '-' => Node::Subtract(Box::new(lhs), Box::new(rhs)),
                            '*' => Node::Multiply(Box::new(lhs), Box::new(rhs)),
                            '/' => Node::Divide(Box::new(lhs), Box::new(rhs)),
                            _ => return Err(ParseError),
                        };
                        value_queue.push(node);
                    } else {
                        // we have an operator but no right hand side value to match
                        return Err(ParseError);
                    }
                }
            }
        }

        let root = value_queue.pop().unwrap();

        // As we consume by operators, we know there's something wrong when there're
        // still values left when we're done with all of them
        if value_queue.len() > 0 {
            return Err(ParseError);
        }
        Ok(SyntaxTree { root })
    }

    pub fn evaluate(self) -> i32 {
        self.root.evaluate()
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

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
            Err(ParseError)
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
            Err(ParseError)
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
}
