#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword(Keyword),
    Symbol(Symbol),

    IntegerConstant(u16),
    StringConstant(String),

    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token: TokenType,
}

impl Token {
    pub fn new(tt: TokenType) -> Self {
        Self { token: tt }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Keyword::Class => "class",
                Keyword::Constructor => "constructor",
                Keyword::Function => "function",
                Keyword::Method => "method",
                Keyword::Field => "field",

                Keyword::Static => "static",
                Keyword::Var => "var",
                Keyword::Int => "int",
                Keyword::Char => "char",
                Keyword::Boolean => "boolean",
                Keyword::Void => "void",

                Keyword::True => "true",
                Keyword::False => "false",
                Keyword::Null => "null",

                Keyword::This => "this",
                Keyword::Let => "let",
                Keyword::Do => "do",

                Keyword::If => "if",
                Keyword::Else => "else",
                Keyword::While => "while",
                Keyword::Return => "return",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(s: char) -> Option<Symbol> {
        let result = match s {
            '(' => Some(Symbol::BracketLeft),
            ')' => Some(Symbol::BracketRight),

            '{' => Some(Symbol::BracketCurlyLeft),
            '}' => Some(Symbol::BracketCurlyRight),

            '[' => Some(Symbol::BracketSquareLeft),
            ']' => Some(Symbol::BracketSquareRight),

            '.' => Some(Symbol::Period),
            ',' => Some(Symbol::Comma),
            ';' => Some(Symbol::SemiColon),
            '+' => Some(Symbol::Plus),
            '-' => Some(Symbol::Minus),
            '*' => Some(Symbol::Times),
            '/' => Some(Symbol::Divide),

            '&' => Some(Symbol::And),
            '|' => Some(Symbol::Or),
            '<' => Some(Symbol::LessThan),
            '>' => Some(Symbol::GreaterThan),
            '=' => Some(Symbol::Equals),
            '~' => Some(Symbol::Not),

            '^' => Some(Symbol::ShiftLeft),
            '#' => Some(Symbol::ShiftRight),

            _ => None,
        };
        return result;
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::BracketLeft => "(",
                Symbol::BracketRight => ")",

                Symbol::BracketCurlyLeft => "{",
                Symbol::BracketCurlyRight => "}",
                Symbol::BracketSquareLeft => "[",
                Symbol::BracketSquareRight => "]",

                Symbol::Period => ".",
                Symbol::Comma => ",",
                Symbol::SemiColon => ";",
                Symbol::Plus => "+",
                Symbol::Minus => "-",
                Symbol::Times => "*",
                Symbol::Divide => "/",

                Symbol::And => "&amp",
                Symbol::Or => "|",
                Symbol::LessThan => "&lt",
                Symbol::GreaterThan => "&gt",
                Symbol::Equals => "=",
                Symbol::Not => "~",

                Symbol::ShiftLeft => "^",
                Symbol::ShiftRight => "#",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    identifier: String,
}

impl Identifier {
    pub fn new(s: &String) -> Self {
        Self {
            identifier: s.to_string(),
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
