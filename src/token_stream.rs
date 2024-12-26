use crate::tokens::{Token, TokenType};

pub struct TokenStream<'a> {
    tokens: std::slice::Iter<'a, Token>,
    current: Option<&'a Token>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut stream = Self {
            tokens: tokens.iter(),
            current: None,
        };
        stream.advance(); // Load the first token
        stream
    }

    pub fn advance(&mut self) -> Result<Token, String> {
        self.current = self.tokens.next();
        return match self.current {
            Some(_) => Ok(self.current.clone()),
            None => Err("Unexpected end of tokens".to_string()),
        };
    }

    pub fn peek(&self) -> Option<&Token> {
        self.current
    }

    pub fn expect(&mut self, expected: &TokenType) -> Result<TokenType, String> {
        if let Some(token) = self.current {
            if &token.token == expected {
                self.advance();
                Ok(token.token.clone())
            } else {
                Err(format!("Expected {:?}, found {:?}", expected, token))
            }
        } else {
            Err("Unexpected end of tokens".to_string())
        }
    }
}
