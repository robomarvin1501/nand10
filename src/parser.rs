use core::panic;

use crate::token_stream::TokenStream;
use crate::tokeniser::tokenise;
use crate::tokens::{Keyword, Symbol, TokenType};

pub fn parse(input_data: String) {
    let tokens = tokenise(input_data);
    let mut output = String::new();
    let mut token_stream: TokenStream = TokenStream::new(&tokens);
    // dbg!(&tokens);
    while let Some(token) = token_stream.peek() {
        if let Err(err) = match token.token.clone() {
            TokenType::Keyword(keyword) => match keyword {
                Keyword::Class => compile_class(&mut token_stream, &mut output),
                _ => panic!("Compilation call to something not the class at the top level"),
            },
            _ => panic!("Compilation call to something not the class at the top level"),
        } {
            eprintln!("ERROR: {}", err);
            break;
        }
    }
}

// Compiles a complete class.
fn compile_class(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "class";
    write_open_tag(TAG, output);

    stream.expect(&TokenType::Keyword(Keyword::Class))?;
    write_token(&Keyword::Class, output);

    // Parse and write the class name
    if let Some(token) = stream.peek() {
        if let TokenType::Identifier(_) = &token.token {
            write_token(&token.token, output);
            stream.advance(); // Consume the class name
        } else {
            return Err(format!("Expected class name, found {:?}", token.token));
        }
    } else {
        return Err("Unexpected end of tokens while parsing class name".to_string());
    }

    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyLeft))?;
    write_token(&Symbol::BracketCurlyLeft, output);

    // Compile class variable declarations (static/field)
    while let Some(token) = stream.peek() {
        match &token.token {
            TokenType::Keyword(Keyword::Static) | TokenType::Keyword(Keyword::Field) => {
                compile_class_var_dec(stream, output)?;
            }
            _ => break, // Exit loop if it's not a class var declaration
        }
    }

    // Compile class constructor/method/function declarations
    while let Some(token) = stream.peek() {
        match &token.token {
            TokenType::Keyword(Keyword::Constructor)
            | TokenType::Keyword(Keyword::Method)
            | TokenType::Keyword(Keyword::Function) => compile_subroutine(stream, output)?,
            _ => break,
        }
    }

    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyRight))?;
    write_token(&Symbol::BracketCurlyRight, output);

    write_close_tag(TAG, output);
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
    output.push_str(&format!("{}\n", Keyword::Do));

    compile_subroutine_call(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    output.push_str(&format!("{}\n", Symbol::SemiColon));

    output.push_str("</doStatement>\n");
    Ok(())
}

// Compiles a subroutine call.
fn compile_subroutine_call(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    // A subroutine call can be of the form:
    // subroutineName(expressionList) OR
    // className|varName.subroutineName(expressionList)

    // Start by checking for an identifier (class/var/subroutine name)
    if let Some(token) = stream.peek() {
        if let TokenType::Identifier(identifier) = &token.token {
            output.push_str(&format!("<identifier> {} </identifier>\n", identifier));
            stream.advance(); // Consume the identifier
        } else {
            return Err(format!("Expected an identifier, found {:?}", token.token));
        }
    } else {
        return Err("Unexpected end of tokens while parsing subroutine call".to_string());
    }

    // Look for a '.' or '(' to determine the form of the subroutine call
    if let Some(token) = stream.peek() {
        match &token.token {
            TokenType::Symbol(Symbol::Period) => {
                // Handle className|varName.subroutineName(expressionList)
                output.push_str(&format!("<symbol> . </symbol>\n"));
                stream.advance(); // Consume '.'

                // Expect another identifier (the subroutine name)
                if let Some(token) = stream.peek() {
                    if let TokenType::Identifier(subroutine_name) = &token.token {
                        output
                            .push_str(&format!("<identifier> {} </identifier>\n", subroutine_name));
                        stream.advance(); // Consume the subroutine name
                    } else {
                        return Err(format!(
                            "Expected a subroutine name after '.', found {:?}",
                            token.token
                        ));
                    }
                } else {
                    return Err("Unexpected end of tokens after '.'".to_string());
                }
            }
            TokenType::Symbol(Symbol::BracketLeft) => {
                // Handle subroutineName(expressionList)
                // Nothing extra needed here
            }
            _ => {
                return Err(format!(
                    "Expected '.' or '(' in subroutine call, found {:?}",
                    token.token
                ));
            }
        }
    }

    // Expect '(' for the parameter list
    if let Err(err) = stream.expect(&TokenType::Symbol(Symbol::BracketLeft)) {
        return Err(format!("Error while parsing subroutine call: {}", err));
    }
    output.push_str(&format!("<symbol> ( </symbol>\n"));

    // Compile the expression list
    compile_expression_list(stream, output)?;

    // Expect ')' to close the parameter list
    if let Err(err) = stream.expect(&TokenType::Symbol(Symbol::BracketRight)) {
        return Err(format!("Error while parsing subroutine call: {}", err));
    }
    output.push_str(&format!("<symbol> ) </symbol>\n"));

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
fn compile_expression_list(
    token_stream: &mut TokenStream,
    output: &mut String,
) -> Result<(), String> {
    Ok(())
}

fn write_open_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("<{}>\n", tag));
}

fn write_close_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("</{}>\n", tag));
}

fn write_token<T: std::fmt::Display>(token: &T, output: &mut String) {
    output.push_str(&format!("{}", token));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do() {
        let raw_jack = String::from("do Hello.world();");
        let tokens = tokenise(raw_jack);
        let mut token_stream: TokenStream = TokenStream::new(&tokens);
        let mut output = String::new();
        let comp = compile_do(&mut token_stream, &mut output);
        assert!(
            comp.is_ok(),
            "compile_do should succeed, but got: {:?}",
            comp
        );
        let expected_output = r"<doStatement>
<keyword> do </keyword>
<identifier> Hello </identifier>
<symbol> . </symbol>
<identifier> world </identifier>
<symbol> ( </symbol>
<symbol> ) </symbol>
<symbol> ; </symbol>
</doStatement>
";
        assert_eq!(
            output, expected_output,
            "Output of compile_do does not match the expected output"
        );
    }

    #[test]
    fn test_class() {
        let raw_jack = String::from(
            "class Test {
static int x;
field boolean y;
constructor Test() { }
function void foo() { }
}",
        );
        let expected_output = String::from(
            "<class>
<keyword> class </keyword>
<identifier> Test </identifier>
<symbol> { </symbol>
<classVarDec>
    <keyword> static </keyword>
    <keyword> int </keyword>
    <identifier> x </identifier>
    <symbol> ; </symbol>
</classVarDec>
<classVarDec>
    <keyword> field </keyword>
    <keyword> boolean </keyword>
    <identifier> y </identifier>
    <symbol> ; </symbol>
</classVarDec>
<subroutineDec>
    <keyword> constructor </keyword>
    <identifier> Test </identifier>
    <symbol> ( </symbol>
    <parameterList> </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
        <symbol> { </symbol>
        <symbol> } </symbol>
    </subroutineBody>
</subroutineDec>
<subroutineDec>
    <keyword> function </keyword>
    <keyword> void </keyword>
    <identifier> foo </identifier>
    <symbol> ( </symbol>
    <parameterList> </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
        <symbol> { </symbol>
        <symbol> } </symbol>
    </subroutineBody>
</subroutineDec>
<symbol> } </symbol>
</class>
",
        );
        let tokens = tokenise(raw_jack);
        let mut token_stream = TokenStream::new(&tokens);
        let mut output = String::new();
        let comp = compile_class(&mut token_stream, &mut output);
        assert!(
            comp.is_ok(),
            "compile_do should succeed, but got: {:?}",
            comp
        );
        assert_eq!(
            output, expected_output,
            "Output of compile_do does not match the expected output"
        );
    }
}
