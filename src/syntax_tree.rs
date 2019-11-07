use token::Token;

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

#[derive(Debug)]
pub struct SyntaxTree {
    root: Node,
}

impl SyntaxTree {
    pub fn from_tokens(tokens: Vec<Token>) -> Result<SyntaxTree, ParseError> {
        let mut value_queue: Vec<Node> = Vec::new();
        let mut operator_queue: Vec<char> = Vec::new();

        for token in tokens {
            match token {
                Token::Integer(digit) => value_queue.push(Node::Value(digit.parse().unwrap())),
                Token::Operator(operator) => operator_queue.push(operator),
            }
        }

        // We assume there are no syntax errors (for now)
        while let Some(operator) = operator_queue.pop() {
            // All operators for now take two arguments
            let (lhs, rhs) = (value_queue.pop().unwrap(), value_queue.pop().unwrap());

            let node = match operator {
                '+' => Node::Add(Box::new(lhs), Box::new(rhs)),
                '-' => Node::Subtract(Box::new(lhs), Box::new(rhs)),
                '*' => Node::Multiply(Box::new(lhs), Box::new(rhs)),
                '/' => Node::Divide(Box::new(lhs), Box::new(rhs)),
                _ => return Err(ParseError),
            };

            value_queue.push(node);
        }

        let root = value_queue.pop().unwrap();
        Ok(SyntaxTree { root })
    }

    pub fn evaluate(self) -> i32 {
        self.root.evaluate()
    }
}

#[derive(Debug)]
pub struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ast_two_plus_two() {
        assert_eq!(
            SyntaxTree::from_tokens(vec![
                Token::Integer(2.to_string()),
                Token::Operator('+'),
                Token::Integer(2.to_string())
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
                Token::Integer(2.to_string()),
                Token::Operator('+'),
                Token::Integer(2.to_string()),
                Token::Operator('+'),
                Token::Integer(2.to_string()),
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
}
