use crate::tokens::{Identifier, Keyword, Symbol, Token, TokenType};
use std::iter::Peekable;

const COMMENT_BEGIN: &'static str = "//";

pub fn tokenise(input_data: String) -> Vec<Token> {
    let whitespaces_cleaned: String = remove_whitespace(input_data);

    let mut current_token: String = String::new();
    let mut chars = whitespaces_cleaned.chars().peekable();

    let mut tokens: Vec<Token> = vec![];

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            // Whitespace ends a token
            if !current_token.is_empty() {
                finalise_token(&mut current_token, &mut tokens);
            }
            continue;
        }

        if let Some(symbol) = Symbol::new(c) {
            // A symbol ends the current token, so finalise it
            if !current_token.is_empty() {
                finalise_token(&mut current_token, &mut tokens);
            }
            // Add the symbol as a new token
            tokens.push(Token::new(TokenType::Symbol(symbol)));
            continue;
        }

        if c == '"' {
            // Handle string constants
            if !current_token.is_empty() {
                finalise_token(&mut current_token, &mut tokens);
            }
            // Collect the entire string constant
            let string_constant = collect_string_constant(&mut chars);
            tokens.push(Token::new(TokenType::StringConstant(string_constant)));
            continue;
        }

        if c.is_digit(10) && current_token.is_empty() {
            // Handle integer constants
            let integer_constant = collect_integer_constant(c, &mut chars);
            tokens.push(Token::new(TokenType::IntegerConstant(integer_constant)));
            continue;
        }

        // Build up the current token
        current_token.push(c);
    }

    // Finalize any remaining token
    if !current_token.is_empty() {
        finalise_token(&mut current_token, &mut tokens);
    }

    return tokens;
}

fn finalise_token(current_token: &mut String, tokens: &mut Vec<Token>) {
    if let Some(keyword) = Keyword::new(current_token) {
        tokens.push(Token::new(TokenType::Keyword(keyword)));
    } else {
        tokens.push(Token::new(TokenType::Identifier(Identifier::new(
            current_token,
        ))));
    }
    current_token.clear();
}

fn collect_string_constant(chars: &mut impl Iterator<Item = char>) -> String {
    let mut string_constant = String::new();
    while let Some(c) = chars.next() {
        if c == '"' {
            break;
        }
        string_constant.push(c);
    }
    string_constant
}

fn collect_integer_constant(start: char, chars: &mut Peekable<impl Iterator<Item = char>>) -> u16 {
    let mut num = start.to_digit(10).unwrap() as u16;

    // Collect digits while they're available
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as u16;
            chars.next(); // Consume the digit
        } else {
            break;
        }
    }

    num
}

fn remove_whitespace(input_data: String) -> String {
    let no_multi_lines = remove_multi_line_comments(input_data);
    let lines: Vec<String> = no_multi_lines
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();
    let mut whitespace_cleaned_lines: Vec<String> = vec![];

    for line in lines {
        if line.is_empty() || line.starts_with(COMMENT_BEGIN) {
        } else if let Some(comment_index) = line.find(COMMENT_BEGIN) {
            let trimmed = &line[..comment_index].trim();
            whitespace_cleaned_lines.push(trimmed.to_string());
        } else {
            whitespace_cleaned_lines.push(line);
        }
    }
    return whitespace_cleaned_lines.concat();
}

fn remove_multi_line_comments(input_data: String) -> String {
    let mut result = String::new();
    let mut in_comment = false;
    let mut chars = input_data.chars().peekable();

    while let Some(c) = chars.next() {
        if in_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                in_comment = false;
                chars.next(); // Consume '/'
            }
        } else {
            if c == '/' && chars.peek() == Some(&'*') {
                in_comment = true;
                chars.next(); // Consume '*'
            } else {
                result.push(c); // Not in comment, so add the character to the result
            }
        }
    }
    return result;
}

#[allow(dead_code)]
pub fn format_tokens_for_display(tokens: Vec<Token>) -> String {
    let mut output: String = String::new();
    for token in tokens {
        output.push_str(&format!("{}\n", token));
    }
    output
}
