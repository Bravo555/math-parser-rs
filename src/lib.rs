mod syntax_tree;
mod token;

use syntax_tree::{ParseError, SyntaxTree};
use token::to_tokens;

pub fn eval(text: &str) -> Result<i32, ParseError> {
    // TODO: infer notation used

    // Build a token list
    let tokens = to_tokens(text);

    // Build a syntax tree
    let tree = SyntaxTree::from_tokens(tokens);
    println!("{:?}", tree);
    // Evaluate the syntax tree
    tree.map(|t| t.evaluate())
}
