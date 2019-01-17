#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operator(char),
    Integer(String),
}

pub fn to_tokens(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    text.chars().for_each(|c| {
        if let Some(digit) = c.to_digit(10) {
            // if previous is a digit add to it and push back
            let prev = tokens.pop();
            if let Some(token) = prev {
                match token {
                    Token::Operator(_) => {
                        tokens.push(token);
                        tokens.push(Token::Integer(digit.to_string()));
                    }
                    Token::Integer(mut prev) => {
                        prev.push(c);
                        tokens.push(Token::Integer(prev));
                    }
                }
            } else {
                tokens.push(Token::Integer(digit.to_string()));
            }
        // else just push new digit
        } else {
            match c {
                '+' => tokens.push(Token::Operator('+')),
                '-' => tokens.push(Token::Operator('-')),
                '*' => tokens.push(Token::Operator('*')),
                '/' => tokens.push(Token::Operator('/')),
                _ => (),
            }
        }
    });

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_plus_two() {
        use self::Token::*;
        assert_eq!(
            to_tokens("2 + 2"),
            vec![
                Integer(2.to_string()),
                Operator('+'),
                Integer(2.to_string())
            ]
        );
    }

    #[test]
    fn multi_integer_numbers() {
        use self::Token::*;
        assert_eq!(
            to_tokens("101 + 202"),
            vec![
                Integer(101.to_string()),
                Operator('+'),
                Integer(202.to_string())
            ]
        );
    }
}
