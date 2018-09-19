mod syntax_tree;
mod token;

use syntax_tree::SyntaxTree;
use token::to_tokens;

pub fn eval(text: &str) -> i32 {
    // Build a token list
    let tokens = to_tokens(text);

    // Build a syntax tree
    let tree = SyntaxTree::from_tokens(tokens);
    println!("{:?}", tree);
    // Evaluate the syntax tree
    tree.evaluate()
}
