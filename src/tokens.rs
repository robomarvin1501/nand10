#[derive(Debug)]
enum TokenType {
    Keyword(Keyword),
    Symbol(Symbol),

    IntegerConstant(u16),
    StringConstant(String),

    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Token {
    token: TokenType,
}

impl Token {
    pub fn new(tt: TokenType) -> Self {
        Self { token: tt }
    }
}

#[derive(Debug)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,

    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,

    True,
    False,
    Null,

    This,
    Let,
    Do,

    If,
    Else,
    While,
    Return,
}

impl Keyword {
    pub fn new(s: &str) -> Option<Keyword> {
        let result = match s {
            "class" => Some(Keyword::Class),
            "constructor" => Some(Keyword::Constructor),
            "function" => Some(Keyword::Function),
            "method" => Some(Keyword::Method),
            "field" => Some(Keyword::Field),

            "static" => Some(Keyword::Static),
            "var" => Some(Keyword::Var),
            "int" => Some(Keyword::Int),
            "char" => Some(Keyword::Char),
            "boolean" => Some(Keyword::Boolean),
            "void" => Some(Keyword::Void),

            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),

            "this" => Some(Keyword::This),
            "let" => Some(Keyword::Let),
            "do" => Some(Keyword::Do),

            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),

            _ => None,
        };
        return result;
    }
}

#[derive(Debug)]
pub enum Symbol {
    BracketLeft,
    BracketRight,

    BracketCurlyLeft,
    BracketCurlyRight,

    BracketSquareLeft,
    BracketSquareRight,

    Period,
    Comma,
    SemiColon,
    Plus,
    Minus,
    Times,
    Divide,

    And,
    Or,
    LessThan,
    GreaterThan,
    Equals,
    Not,

    ShiftLeft,  // ^
    ShiftRight, // #
}

impl Symbol {
    pub fn new(s: &str) -> Option<Symbol> {
        let result = match s {
            "(" => Some(Symbol::BracketLeft),
            ")" => Some(Symbol::BracketRight),

            "{" => Some(Symbol::BracketCurlyLeft),
            "}" => Some(Symbol::BracketCurlyRight),

            "[" => Some(Symbol::BracketSquareLeft),
            "]" => Some(Symbol::BracketSquareRight),

            "." => Some(Symbol::Period),
            "," => Some(Symbol::Comma),
            ";" => Some(Symbol::SemiColon),
            "+" => Some(Symbol::Plus),
            "-" => Some(Symbol::Minus),
            "*" => Some(Symbol::Times),
            "/" => Some(Symbol::Divide),

            "&" => Some(Symbol::And),
            "|" => Some(Symbol::Or),
            "<" => Some(Symbol::LessThan),
            ">" => Some(Symbol::GreaterThan),
            "=" => Some(Symbol::Equals),
            "~" => Some(Symbol::Not),

            "^" => Some(Symbol::ShiftLeft),
            "#" => Some(Symbol::ShiftRight),

            _ => None,
        };
        return result;
    }
}

#[derive(Debug)]
pub struct Identifier {
    identifier: String,
}

impl Identifier {
    pub fn new(s: String) -> Self {
        Self { identifier: s }
    }
}
