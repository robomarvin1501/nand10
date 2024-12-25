const COMMENT_BEGIN: &'static str = "//";
const MULTI_LINE_COMMENT_BEGIN: &'static str = "/*";
const MULTI_LINE_COMMENT_END: &'static str = "*/";


/*
 *
    - keyword: 'class' | 'constructor' | 'function' | 'method' | 'field' |
               'static' | 'var' | 'int' | 'char' | 'boolean' | 'void' | 'true' |
               'false' | 'null' | 'this' | 'let' | 'do' | 'if' | 'else' |
               'while' | 'return'
    - symbol: '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' |
              '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' | '~' | '^' | '#'
 */

enum TokenType {
    Keyword,
    Symbol,


}

pub #[derive(Debug)]
struct Token {
    type: TokenType
}

pub fn tokenise(input_data: String) {
    let whitespaces_cleaned: String = remove_whitespace(input_data);

    println!("{}", whitespaces_cleaned);
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
                chars.next();
            }
        } else {
            if c == '/' && chars.peek() == Some(&'/') {
                in_comment = true;
                chars.next();
            } else {
                result.push(c);
            }
        }
    }
    return result;
}
