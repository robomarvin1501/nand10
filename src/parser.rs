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
fn compile_class_var_dec(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "classVarDec";
    write_open_tag(TAG, output);

    // parse kind (static or field)
    let kind = parse_keyword(stream, &[Keyword::Static, Keyword::Field])?;

    // parse type (int, char, boolean, class name)
    let var_type = parse_type(stream, false)?;

    // parse variable name
    let mut var_name = parse_identifier(stream)?;

    // output variable xml
    write_token(&kind, output);
    write_token(&var_type, output);
    write_token(&var_name, output);

    while matches!(stream.peek(), Some(token) if token.token == TokenType::Symbol(Symbol::Comma)) {
        stream.advance();
        write_token(&Symbol::Comma, output);

        var_name = parse_identifier(stream)?;
        write_token(&var_name, output);
    }
    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    write_token(&Symbol::SemiColon, output);

    write_close_tag(TAG, output);
    Ok(())
}

// Helper to parse a keyword from a list of valid keywords
fn parse_keyword(
    stream: &mut TokenStream,
    valid_keywords: &[Keyword],
) -> Result<TokenType, String> {
    if let Some(token) = stream.advance_prev() {
        if let TokenType::Keyword(keyword) = &token.token {
            if valid_keywords.contains(&keyword) {
                return Ok(token.token.clone());
            }
        }
        Err(format!(
            "Expected one of {:?}, found {:?}",
            valid_keywords, token.token
        ))
    } else {
        Err("Unexpected end of tokens".to_string())
    }
}

// Helper to parse a type (int, char, boolean, or class name)
fn parse_type(stream: &mut TokenStream, allow_void: bool) -> Result<TokenType, String> {
    if let Some(token) = stream.advance_prev() {
        match &token.token {
            TokenType::Keyword(Keyword::Int)
            | TokenType::Keyword(Keyword::Char)
            | TokenType::Keyword(Keyword::Boolean)
            | TokenType::Identifier(_) => Ok(token.token.clone()),

            TokenType::Keyword(Keyword::Void) if allow_void => Ok(token.token.clone()),

            _ => Err(format!("Expected a type, found {:?}", token.token)),
        }
    } else {
        Err("Unexpected end of tokens".to_string())
    }
}

// Helper to parse an identifier
fn parse_identifier(stream: &mut TokenStream) -> Result<TokenType, String> {
    if let Some(token) = stream.advance_prev() {
        if let TokenType::Identifier(_) = token.token {
            Ok(token.token.clone())
        } else {
            Err(format!("Expected an identifier, found {:?}", token.token))
        }
    } else {
        Err("Unexpected end of tokens".to_string())
    }
}

// Helper to parse an operator from a list of operators
fn parse_operator(stream: &mut TokenStream, valid_symbols: &[Symbol]) -> Result<TokenType, String> {
    if let Some(token) = stream.peek() {
        if let TokenType::Symbol(symbol) = &token.token {
            if valid_symbols.contains(&symbol) {
                return Ok(token.token.clone());
            }
        }
        Err(format!(
            "Expected one of {:?}, found {:?}",
            valid_symbols, token.token
        ))
    } else {
        Err("Unexpected end of tokens".to_string())
    }
}

// Compiles a complete method, function, or constructor.
//      You can assume that classes with constructors have at least one field,
//      you will understand why this is necessary in project 11.
fn compile_subroutine(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "subroutineDec";
    write_open_tag(TAG, output);

    let kind = parse_keyword(
        stream,
        &[Keyword::Constructor, Keyword::Function, Keyword::Method],
    )?;
    write_token(&kind, output);
    let return_type = parse_type(stream, true)?;
    write_token(&return_type, output);

    let function_name = parse_identifier(stream)?;
    write_token(&function_name, output);

    stream.expect(&TokenType::Symbol(Symbol::BracketLeft))?;
    write_token(&TokenType::Symbol(Symbol::BracketLeft), output);

    compile_parameter_list(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::BracketRight))?;
    write_token(&TokenType::Symbol(Symbol::BracketRight), output);

    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyLeft))?;
    write_token(&TokenType::Symbol(Symbol::BracketCurlyLeft), output);

    while matches!(stream.peek(), Some(token) if token.token == TokenType::Keyword(Keyword::Var)) {
        compile_var_dec(stream, output)?;
    }

    compile_statements(stream, output)?;
    compile_return(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyRight))?;
    write_token(&TokenType::Symbol(Symbol::BracketCurlyRight), output);

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a (possibly empty) parameter list, not including the
//      enclosing "()".
fn compile_parameter_list(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "parameterList";
    write_open_tag(TAG, output);
    if let Some(token) = stream.peek() {
        if token.token == TokenType::Symbol(Symbol::BracketRight) {
            write_close_tag(TAG, output);
            return Ok(());
        }

        let mut arg_type = parse_type(stream, false)?;
        write_token(&arg_type, output);

        let mut arg_name = parse_identifier(stream)?;
        write_token(&arg_name, output);

        while matches!(stream.peek(), Some(token) if token.token == TokenType::Symbol(Symbol::Comma))
        {
            stream.advance();
            write_token(&Symbol::Comma, output);

            arg_type = parse_type(stream, false)?;
            write_token(&arg_type, output);
            arg_name = parse_identifier(stream)?;
            write_token(&arg_name, output);
        }
    } else {
        return Err("Unexpected end of tokens when compiling parameter list".to_string());
    }
    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a var declaration.
fn compile_var_dec(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    stream.expect(&TokenType::Keyword(Keyword::Var))?;
    write_token(&TokenType::Keyword(Keyword::Var), output);

    let var_type = parse_type(stream, false)?;
    write_token(&var_type, output);

    let mut var_name = parse_identifier(stream)?;
    write_token(&var_name, output);

    while matches!(stream.peek(), Some(token) if token.token == TokenType::Symbol(Symbol::Comma)) {
        stream.advance();
        write_token(&Symbol::Comma, output);

        var_name = parse_identifier(stream)?;
        write_token(&var_name, output);
    }

    Ok(())
}

// Compiles a sequence of statements, not including the enclosing "}".
fn compile_statements(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "statements";
    write_open_tag(TAG, output);

    while let Some(token) = stream.peek() {
        match token.token {
            TokenType::Keyword(Keyword::Let) => compile_let(stream, output)?,
            TokenType::Keyword(Keyword::If) => compile_if(stream, output)?,
            TokenType::Keyword(Keyword::While) => compile_while(stream, output)?,
            TokenType::Keyword(Keyword::Do) => compile_do(stream, output)?,
            TokenType::Keyword(Keyword::Return) => break,

            _ => break,
        }
    }

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a do statement.
fn compile_do(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "doStatement";
    write_open_tag(TAG, output);

    stream.expect(&TokenType::Keyword(Keyword::Do))?;
    write_token(&Keyword::Do, output);

    compile_subroutine_call(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    write_token(&Symbol::SemiColon, output);

    write_close_tag(TAG, output);
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
            write_token(&identifier, output);
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
                write_token(&Symbol::Period, output);
                stream.advance(); // Consume '.'

                // Expect another identifier (the subroutine name)
                if let Some(token) = stream.peek() {
                    if let TokenType::Identifier(subroutine_name) = &token.token {
                        write_token(subroutine_name, output);
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
    write_token(&Symbol::BracketLeft, output);

    // Compile the expression list
    compile_expression_list(stream, output)?;

    // Expect ')' to close the parameter list
    if let Err(err) = stream.expect(&TokenType::Symbol(Symbol::BracketRight)) {
        return Err(format!("Error while parsing subroutine call: {}", err));
    }
    write_token(&Symbol::BracketRight, output);

    Ok(())
}

// Compiles a let statement.
fn compile_let(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "letStatement";
    write_open_tag(TAG, output);
    stream.expect(&TokenType::Keyword(Keyword::Let))?;
    write_token(&Keyword::Let, output);

    let var_name = parse_identifier(stream)?;
    write_token(&var_name, output);

    while matches!(stream.peek(), Some(token) if token.token == TokenType::Symbol(Symbol::BracketSquareLeft))
    {
        stream.expect(&TokenType::Symbol(Symbol::BracketSquareLeft))?;
        write_token(&Symbol::BracketSquareLeft, output);
        compile_expression(stream, output)?;
        stream.expect(&TokenType::Symbol(Symbol::BracketSquareRight))?;
        write_token(&Symbol::BracketSquareRight, output);
    }

    stream.expect(&TokenType::Symbol(Symbol::Equals))?;
    write_token(&TokenType::Symbol(Symbol::Equals), output);

    compile_expression(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    write_token(&Symbol::SemiColon, output);

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a while statement.
fn compile_while(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "whileStatement";
    write_open_tag(TAG, output);

    stream.expect(&TokenType::Keyword(Keyword::While))?;
    write_token(&Keyword::While, output);

    // while condition
    stream.expect(&TokenType::Symbol(Symbol::BracketLeft))?;
    write_token(&Symbol::BracketLeft, output);

    compile_expression(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::BracketRight))?;
    write_token(&Symbol::BracketRight, output);

    // while body
    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyRight))?;
    write_token(&Symbol::BracketCurlyRight, output);

    compile_statements(stream, output)?;

    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyLeft))?;
    write_token(&Symbol::BracketCurlyLeft, output);

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a return statement.
fn compile_return(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "returnStatement";
    write_open_tag(TAG, output);

    stream.expect(&TokenType::Keyword(Keyword::Return))?;
    write_token(&Keyword::Return, output);

    if let Some(token) = stream.peek() {
        if token.token != TokenType::Symbol(Symbol::SemiColon) {
            compile_expression(stream, output)?;
        }
    } else {
        return Err("Unexpected end of tokens when compiling return".to_string());
    }

    stream.expect(&TokenType::Symbol(Symbol::SemiColon))?;
    write_token(&Symbol::SemiColon, output);

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a if statement, possibly with a trailing else clause.
fn compile_if(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "ifStatement";
    write_open_tag(TAG, output);

    // If and opening bracket
    stream.expect(&TokenType::Keyword(Keyword::If))?;
    write_token(&Keyword::If, output);
    stream.expect(&TokenType::Symbol(Symbol::BracketLeft))?;
    write_token(&Symbol::BracketLeft, output);
    // brackets contents
    compile_expression(stream, output)?;
    // closing bracket
    stream.expect(&TokenType::Symbol(Symbol::BracketRight))?;
    write_token(&Symbol::BracketRight, output);

    // Body open
    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyLeft))?;
    write_token(&Symbol::BracketCurlyLeft, output);
    // Body
    compile_statements(stream, output)?;
    // Body close
    stream.expect(&TokenType::Symbol(Symbol::BracketCurlyRight))?;
    write_token(&Symbol::BracketCurlyRight, output);

    if let Some(token) = stream.peek() {
        if token.token == TokenType::Keyword(Keyword::Else) {
            stream.expect(&TokenType::Keyword(Keyword::Else))?;
            write_token(&Keyword::Else, output);
            // Body open
            stream.expect(&TokenType::Symbol(Symbol::BracketCurlyLeft))?;
            write_token(&Symbol::BracketCurlyLeft, output);
            // Body
            compile_statements(stream, output)?;
            // Body close
            stream.expect(&TokenType::Symbol(Symbol::BracketCurlyRight))?;
            write_token(&Symbol::BracketCurlyRight, output);
        }
    } else {
        return Err("Unexpected end of tokens when compiling if".to_string());
    }

    write_close_tag(TAG, output);

    Ok(())
}

// Compiles an expression.
fn compile_expression(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "expression";
    write_open_tag(TAG, output);

    compile_term(stream, output)?;

    let op = parse_operator(
        stream,
        &[
            Symbol::Plus,
            Symbol::Minus,
            Symbol::Divide,
            Symbol::And,
            Symbol::Or,
            Symbol::LessThan,
            Symbol::GreaterThan,
            Symbol::Equals,
            Symbol::ShiftLeft,
            Symbol::ShiftRight,
        ],
    );

    match op {
        Ok(operator) => {
            stream.advance();
            stream.expect(&operator)?;
            write_token(&operator, output);
            compile_term(stream, output)?;
        }
        Err(_) => {}
    }

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a term.
// This routine is faced with a slight difficulty when trying to decide between
// some of the alternative parsing rules. Specifically, if the current token is
// an identifier, the routing must distinguish between a variable, an array
// entry, and a subroutine call. A single look-ahead token, which may be one
// of "[", "(", or "." suffices to distinguish between the three possibilities.
// Any other token is not part of this term and should not be advanced over.
fn compile_term(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "term";
    write_open_tag(TAG, output);

    if let Some(token) = stream.peek() {
        match &token.token {
            // Handle constants (integer and string literals)
            TokenType::IntegerConstant(_) | TokenType::StringConstant(_) => {
                write_token(&token.token, output);
                stream.advance().unwrap();
            }

            // Handle keyword constants (true, false, null, this)
            TokenType::Keyword(keyword)
                if matches!(
                    keyword,
                    Keyword::True | Keyword::False | Keyword::Null | Keyword::This
                ) =>
            {
                write_token(&token.token, output);
                stream.advance().unwrap();
            }

            // Handle unary operators followed by a term (-term | ~term | ^term | #term)
            TokenType::Symbol(Symbol::Minus)
            | TokenType::Symbol(Symbol::Not)
            | TokenType::Symbol(Symbol::ShiftLeft)
            | TokenType::Symbol(Symbol::ShiftRight) => {
                let operator = token.token.clone(); // Save the operator
                write_token(&operator, output);
                stream.advance().unwrap();
                compile_term(stream, output)?; // Compile the term
            }

            // Handle expressions in parentheses: (expression)
            TokenType::Symbol(Symbol::BracketLeft) => {
                write_token(&token.token, output); // Write '('
                stream.advance().unwrap();
                compile_expression(stream, output)?; // Compile the inner expression
                stream.expect(&TokenType::Symbol(Symbol::BracketRight))?;
                write_token(&TokenType::Symbol(Symbol::BracketRight), output); // Write ')'
            }

            // Handle identifiers (variable, array entry, or subroutine call)
            TokenType::Identifier(_) => {
                let identifier = token.token.clone(); // Save the identifier
                write_token(&identifier, output);
                stream.advance().unwrap(); // Advance past the identifier

                if let Some(next_token) = stream.peek() {
                    match next_token.token {
                        TokenType::Symbol(Symbol::BracketSquareLeft) => {
                            // Array entry: varName[expression]
                            write_token(&TokenType::Symbol(Symbol::BracketSquareLeft), output);
                            stream.advance().unwrap(); // Consume '['
                            compile_expression(stream, output)?; // Compile the expression
                            stream.expect(&TokenType::Symbol(Symbol::BracketSquareRight))?;
                            write_token(&TokenType::Symbol(Symbol::BracketSquareRight), output);
                        }
                        TokenType::Symbol(Symbol::BracketLeft)
                        | TokenType::Symbol(Symbol::Period) => {
                            // Subroutine call: subroutineName(expressionList) or
                            // className.varName.subroutineName(expressionList)
                            compile_subroutine_call(stream, output)?;
                        }
                        _ => {
                            // Otherwise, it's just a variable (nothing more to process)
                        }
                    }
                }
            }

            _ => {
                return Err(format!(
                    "Unexpected token {:?} when compiling term",
                    token.token
                ));
            }
        }
    } else {
        return Err("Unexpected end of tokens when compiling term".to_string());
    }

    write_close_tag(TAG, output);
    Ok(())
}

// Compiles a (possibly empty) comma-separated list of expressions.
fn compile_expression_list(stream: &mut TokenStream, output: &mut String) -> Result<(), String> {
    const TAG: &str = "expressionList";

    if let Some(token) = stream.peek() {
        if !matches!(token.token, TokenType::Symbol(Symbol::BracketRight)) {
            // There is at least one expression, so compile it
            write_open_tag(TAG, output);
            compile_expression(stream, output)?;

            // Handle any additional comma-separated expressions
            while let Some(token) = stream.peek() {
                if token.token == TokenType::Symbol(Symbol::Comma) {
                    stream.expect(&TokenType::Symbol(Symbol::Comma))?;
                    write_token(&TokenType::Symbol(Symbol::Comma), output);

                    compile_expression(stream, output)?;
                } else {
                    break; // No more expressions
                }
            }
            write_close_tag(TAG, output);
        }
    } else {
        return Err("Unexpected end of tokens when compiling expression list".to_string());
    }

    Ok(())
}

fn write_open_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("<{}>\n", tag));
}

fn write_close_tag(tag: &str, output: &mut String) {
    output.push_str(&format!("</{}>\n", tag));
}

fn write_token<T: std::fmt::Display>(token: &T, output: &mut String) {
    output.push_str(&format!("{}\n", token));
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
field char a, b;
constructor Test new() { return this; }
function void foo() { return; }
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
<classVarDec>
    <keyword> field </keyword>
    <keyword> char </keyword>
    <identifier> a </identifier>
    <symbol> , </symbol>
    <identifier> b </identifier>
    <symbol> ; </symbol>
</classVarDec>
<subroutineDec>
    <keyword> constructor </keyword>
    <identifier> Test </identifier>
    <identifier> new </identifier>
    <symbol> ( </symbol>
    <parameterList> </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
        <symbol> { </symbol>
            <returnStatement>
                <identifier> this </identifier>
            </returnStatement>
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
