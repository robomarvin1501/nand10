use crate::token_stream::TokenStream;
use crate::tokeniser::tokenise;
use crate::tokens::{Keyword, Symbol, Token, TokenType};

pub fn parse(input_data: String) {
    let tokens = tokenise(input_data);
    let mut output = String::new();
    let mut token_stream: TokenStream = TokenStream::new(&tokens);
    // dbg!(&tokens);
    while let Some(token) = token_stream.peek() {
        if let Err(err) = match token.token.clone() {
            TokenType::Keyword(keyword) => match keyword {
                Keyword::Class => compile_class(),
                Keyword::Constructor | Keyword::Function | Keyword::Method => {
                    compile_subroutine(&mut token_stream, &mut output)
                }
                Keyword::Field | Keyword::Static => compile_class_var_dec(),
                Keyword::Var => compile_var_dec(),
                Keyword::Int
                | Keyword::Char
                | Keyword::Boolean
                | Keyword::Void
                | Keyword::True
                | Keyword::False
                | Keyword::Null
                | Keyword::This => {
                    panic!("{} should not be consumed by the keyword level", keyword)
                }
                Keyword::Let => compile_let(),
                Keyword::Do => compile_do(&mut token_stream, &mut output),
                Keyword::If => compile_if(),
                Keyword::Else => panic!("{} should have been consumed by if", keyword),
                Keyword::While => compile_while(),
                Keyword::Return => compile_return(),
            },
            TokenType::Symbol(s) => Ok(()),
            TokenType::IntegerConstant(i) => Ok(()),
            TokenType::StringConstant(s) => Ok(()),
            TokenType::Identifier(identifier) => Ok(()),
        } {
            eprintln!("ERROR: {}", err);
            break;
        }
    }
}

// Compiles a complete class.
fn compile_class() -> Result<(), String> {
    Ok(())
}

// Compiles a static declaration or a field declaration.
fn compile_class_var_dec() -> Result<(), String> {
    Ok(())
}

// Compiles a complete method, function, or constructor.
//      You can assume that classes with constructors have at least one field,
//      you will understand why this is necessary in project 11.
fn compile_subroutine(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    Ok(())
}

// Compiles a (possibly empty) parameter list, not including the
//      enclosing "()".
fn compile_parameter_list(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    Ok(())
}

// Compiles a var declaration.
fn compile_var_dec() -> Result<(), String> {
    Ok(())
}

// Compiles a sequence of statements, not including the enclosing "{Ok(())}".
fn compile_statements() -> Result<(), String> {
    Ok(())
}

// Compiles a do statement.
fn compile_do(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    output.push_str("<doStatement>\n");

    stream.expect(&TokenType::Keyword(Keyword::Do))?;
    output.push_str(&format!("{}", Keyword::Do));

    compile_subroutine_call(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    output.push_str(&format!("{}", Symbol::SemiColon));

    output.push_str("</doStatement>\n");
    Ok(())
}

// Compiles a let statement.
fn compile_let() -> Result<(), String> {
    Ok(())
}

// Compiles a while statement.
fn compile_while() -> Result<(), String> {
    Ok(())
}

// Compiles a return statement.
fn compile_return() -> Result<(), String> {
    Ok(())
}

// Compiles a if statement, possibly with a trailing else clause.
fn compile_if() -> Result<(), String> {
    Ok(())
}

// Compiles an expression.
fn compile_expression() -> Result<(), String> {
    Ok(())
}

// Compiles a term.
// This routine is faced with a slight difficulty when trying to decide between
// some of the alternative parsing rules. Specifically, if the current token is
// an identifier, the routing must distinguish between a variable, an array
// entry, and a subroutine call. A single look-ahead token, which may be one
// of "[", "(", or "." suffices to distinguish between the three possibilities.
// Any other token is not part of this term and should not be advanced over.
fn compile_term() -> Result<(), String> {
    Ok(())
}

// Compiles a (possibly empty) comma-separated list of expressions.
fn compile_expression_list(token_stream: &mut TokenStream) -> Result<(), String> {
    Ok(())
}

fn write_open_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("<{}>\n", tag));
}

fn write_close_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("</{}>\n", tag));
}

fn write_token(token: &Token, output: &mut String) {
    output.push_str(&format!("{}", token));
}
