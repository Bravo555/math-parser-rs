use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operator(char),
    Integer(i32),
}

pub struct Tokens<'a> {
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
            println!("{}", remainder);
            Some(token)
        })
    }
}

pub fn to_tokens(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = text.chars().skip_while(|c| c.is_whitespace()).peekable();

    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut parsed: Vec<char> = vec![chars.next().unwrap()];
                while let Some(next_char) = chars.peek() {
                    if !next_char.is_digit(10) {
                        break;
                    }
                    parsed.push(chars.next().unwrap());
                }
                tokens.push(Token::Integer(
                    parsed.iter().collect::<String>().parse::<i32>().unwrap(),
                ));
            }
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(chars.next().unwrap())),
            _ if c.is_whitespace() => {
                chars.next();
            }
            _ => panic!("wrong token"),
        }
    }
    tokens
}

fn parse_token(text: &str) -> Option<(Token, &str)> {
    // skip initial whitespaces, if present

    None
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

    #[test]
    fn works_for_not_space_separated_input() {
        assert_eq!(
            to_tokens("2+2+2+2"),
            vec![
                Integer(2),
                Operator('+'),
                Integer(2),
                Operator('+'),
                Integer(2),
                Operator('+'),
                Integer(2),
            ]
        )
    }
}
