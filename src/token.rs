#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operator(char),
    Digit(i32),
}

pub fn to_tokens(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    text.chars().for_each(|c| {
        if let Some(digit) = c.to_digit(10) {
            tokens.push(Token::Digit(digit as i32));
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
        assert_eq!(to_tokens("2 + 2"), vec![Digit(2), Operator('+'), Digit(2)]);
    }

    #[test]
    fn multi_digit_numbers() {
        use self::Token::*;
        assert_eq!(
            to_tokens("101 + 202"),
            vec![Digit(101), Operator('+'), Digit(202)]
        );
    }
}
