use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operator(char),
    Integer(i32),
}

struct Tokens<'a> {
    text: &'a str,
}

impl<'a> Tokens<'a> {
    fn new(text: &str) -> Tokens {
        Tokens { text }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        parse_token(self.text).and_then(|(token, remainder)| {
            self.text = remainder;
            Some(token)
        })
    }
}

pub fn to_tokens(text: &str) -> Vec<Token> {
    Tokens::new(text).collect()
}

fn parse_token(text: &str) -> Option<(Token, &str)> {
    let mut parsed: Vec<char> = Vec::new();
    let mut chars = text.chars();

    // skip initial whitespaces, if present
    match chars.by_ref().skip_while(|c| c.is_whitespace()).next() {
        Some(c) => match c {
            '0'..='9' => {
                parsed.push(c);
                parsed.extend(chars.by_ref().take_while(|c| c.is_digit(10)));
                Some((
                    Token::Integer(parsed.into_iter().collect::<String>().parse().unwrap()),
                    chars.as_str(),
                ))
            }
            '+' | '-' | '*' | '/' => Some((Token::Operator(c), chars.as_str())),
            _ => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use self::Token::*;
    use super::*;
    #[test]
    fn two_plus_two() {
        assert_eq!(
            to_tokens("2 + 2"),
            vec![Integer(2), Operator('+'), Integer(2)]
        );
    }

    #[test]
    fn multi_integer_numbers() {
        assert_eq!(
            to_tokens("101 + 202"),
            vec![Integer(101), Operator('+'), Integer(202)]
        );
    }

    #[test]
    fn space_separated_tokens() {
        assert_eq!(
            to_tokens("2 + 2 + 2 2 2"),
            vec![
                Integer(2),
                Operator('+'),
                Integer(2),
                Operator('+'),
                Integer(2),
                Integer(2),
                Integer(2),
            ]
        );
    }

    #[test]
    fn handle_whitespace() {
        assert_eq!(
            to_tokens("    231\n    \t+\n    1312    "),
            vec![Integer(231), Operator('+'), Integer(1312)]
        );
    }
}
