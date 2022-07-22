use std::fmt;
use std::borrow::Borrow;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Token{
    EOF, 
    Illegal,
    Identifier(SourceLocation,String),

    StringLiteral(SourceLocation, String),
    CharLiteral(SourceLocation, char),
    // parentheses
    OpenDelimiter(SourceLocation, Delimiter),
    CloseDelimiter(SourceLocation, Delimiter),
    // keywords
    Node(SourceLocation),
    If(SourceLocation),
    Else(SourceLocation),
    True(SourceLocation),
    False(SourceLocation),

    AndAlso(SourceLocation),    // and
    OrElse(SourceLocation),     // or

    // operators
    Assign(SourceLocation),        // =
    Dot(SourceLocation),           // .
    Comma(SourceLocation),         // ,
    Slash(SourceLocation),         // /
    Plus(SourceLocation),          // +
    Dash(SourceLocation),          // -
    Asterisk(SourceLocation),      // *
    Bang(SourceLocation),          // !  
    Pipe(SourceLocation),          // | 
    LessThan(SourceLocation),      // <
    GreaterThan(SourceLocation),   // >
    Arrow(SourceLocation)          // =>
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Delimiter{
    Brace,
    Paren,
    Bracket
}

impl Token{
    pub fn to_string(&self) -> String{
        match self{
            Token::EOF => String::from("\0"),
            Token::Identifier(_, txt) => txt.to_string(),
            Token::StringLiteral(_,txt) => txt.to_string(),
            Token::CharLiteral(_,ch) => ch.to_string(),
            Token::OpenDelimiter(_, Delimiter::Brace) => String::from("{"),
            Token::OpenDelimiter(_, Delimiter::Paren) => String::from("("),
            Token::OpenDelimiter(_, Delimiter::Bracket) => String::from("["),
            Token::CloseDelimiter(_, Delimiter::Brace) => String::from("{"),
            Token::CloseDelimiter(_, Delimiter::Paren) => String::from("("),
            Token::CloseDelimiter(_, Delimiter::Bracket) => String::from("["),
            Token::Node(_) => String::from("node"),
            Token::If(_) => String::from("if"),
            Token::Else(_) => String::from("else"),
            Token::True(_) => String::from("true"),
            Token::False(_) => String::from("false"),
            Token::AndAlso(_) => String::from("and"),
            Token::OrElse(_) => String::from("or"),
            Token::Assign(_) => String::from("="),
            Token::Arrow(_) => String::from("=>"),
            Token::Dot(_) => String::from("."),
            Token::Comma(_) => String::from("),"),
            Token::Slash(_) => String::from("/"),
            Token::Plus(_) => String::from("+"),
            Token::Dash(_) => String::from("-"),
            Token::Asterisk(_) => String::from("*"),
            Token::LessThan(_) => String::from("<"),
            Token::GreaterThan(_) => String::from(">"),
            Token::Bang(_) => String::from("!"),
            Token::Pipe(_) => String::from("|"),
            _ => String::from("\0")
        }
    }
}

#[derive(PartialEq)]
#[derive(Clone,Copy)]
pub struct SourceLocation{
    pub line: usize,
    pub col: usize
}

impl SourceLocation{
    pub fn new(line:usize, col:usize) -> Self{Self{line,col}}
}

impl fmt::Debug for SourceLocation{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "l:{:?}:{:?}",self.line,self.col)
    }
}
