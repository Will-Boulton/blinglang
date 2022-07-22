use std::str::Chars;
use self::token::Token;
use self::token::SourceLocation;
use self::token::Delimiter;
use crate::lexing::literal::LiteralMode;

pub mod literal;
pub mod token;

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(input);
    std::iter::from_fn( move || {
        match lexer.next_token() {
            Token::EOF => None,
            x => Some(x)
        }
    })
}

pub(crate) struct Lexer <'a>{
    input:  Chars<'a>,
    pos: usize,
    read_pos: usize,
    line_pos: usize,
    line:usize,
    ch: char,
    prev_token: Option<Token>
}

/// Lexer for blinglang 
impl<'a> Lexer <'a> {
    pub(self) fn new(input: &'a str) -> Self{
        Self { 
            input: input.chars(),
            pos: 0,
            read_pos: 0,
            line_pos: 0,
            line: 0, 
            ch: EOF_CHAR, 
            prev_token: None 
        }
    }

    fn location(&self) -> SourceLocation{
        SourceLocation::new(self.line, self.line_pos)
    }

    fn advance_pos(&mut self){
        self.input.next();
        self.pos = self.read_pos;
        self.read_pos = self.read_pos + 1;
        self.line_pos = self.line_pos + 1;
    }

    fn read_char(&mut self) -> Option<char>{
        loop{
            let next_char = self.peek_next();
            match next_char{
                Some(ch) => {
                    self.advance_pos();
                    if !is_whitespace(ch) {
                        self.ch = ch;
                        return Some(ch)
                    }
                    if ch == '\n' {
                        self.line = self.line+1;
                        self.line_pos = 0;
                    }
                }
                None => return None
            }
        }
    }

    /// check the next character
    fn peek_next(&mut self) -> Option<char>{ 
        return self.input.clone().next();
    }  
    
    /// check the next character
    fn peek_next_next(&mut self) -> Option<char>{ 
        let mut x = self.input.clone();
        x.next();
        return x.next();
    } 

    fn read_identifier(&mut self) -> Vec<char> {
        let mut v = Vec::new();
        v.push(self.ch);
        loop{
            match self.peek_next(){
                Some(ch) => {
                    if is_valid_identifier_char(ch) { 
                        self.read_char();
                        v.push(ch);
                    } else { break; }
                },
                None => {break;}
            }
        }
        return v;
    }

    pub fn next_token(&mut self) -> Token {
        let next_char = self.read_char();
        let loc = self.location();
        let tok = match next_char{
            Some(ch) => match ch {
                    '.' => Token::Dot(loc),
                    ',' => Token::Comma(loc),
                    '=' => {
                        let collect_arrow = |c,_| c != '>';
                        match self.lookahead_collect(collect_arrow).as_deref()
                        {
                            Some("=>") => Token::Arrow(loc),
                            Some(other) => panic!("{:?}",other),
                            None => Token::Assign(loc)
                        }
                    }
                    '('=> Token::OpenDelimiter(loc,Delimiter::Paren),
                    ')'=> Token::CloseDelimiter(loc, Delimiter::Paren),
                    '{'=> Token::OpenDelimiter(loc,Delimiter::Brace),
                    '}'=> Token::CloseDelimiter(loc, Delimiter::Brace),
                    '['=> Token::OpenDelimiter(loc,Delimiter::Bracket),
                    ']'=> Token::CloseDelimiter(loc, Delimiter::Bracket),
                    '+'=> Token::Plus(loc),
                    '-'=> Token::Dash(loc),
                    '/'=> Token::Slash(loc),
                    '*'=> Token::Asterisk(loc),
                    '!'=> Token::Bang(loc),
                    '>' => Token::GreaterThan(loc),
                    '<' => Token::LessThan(loc),
                    '|' => Token::Pipe(loc),
                    EOF_CHAR => Token::EOF,
                    x => {
                        if(x.is_literal_mode()){
                            match literal::lex_literal(self){
                                Ok(t) => t,
                                _ => Token::Illegal
                            }
                        } else {
                            let ident = self.read_identifier();
                            match match_keyword(&ident, loc) {
                                Ok(tok) => tok,
                                Err(_) => Token::Identifier(loc, ident.into_iter().collect())
                            }
                        }
                    } 
                },
            None => Token::EOF
        };
        self.prev_token = Some(tok.clone());
        return tok;
    }

    pub(crate) fn skip_to_next_whitespace(&mut self){
        self.skip_while(|c| !c.is_whitespace());
    }

    pub(crate) fn skip_while(&mut self, pred : fn(char) -> bool){
        loop{
            match self.peek_next(){
                Some(ch) if !pred(ch) => break,
                _ => {}
            }
        }
    }

    pub fn lookahead_collect(&mut self, stop_pred : fn(char,char) -> bool) -> Option<String>{
        let mut vec = Vec::new();
        vec.push(self.ch);
        loop {
            let chr = self.peek_next();
            match chr{
                Some(chr) => {
                    if !is_whitespace(chr) && !stop_pred(chr, vec[vec.len()]){
                        vec.push(chr);    
                        self.read_char();
                    } else {break;}
                },
                None => {break;}
            }
        }
        if vec.len() > 1{
            return Some(vec.into_iter().collect());
        } else {
            return None;
        }

    }

    
}

fn is_whitespace(ch:char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

fn is_letter(ch: char) -> bool{
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z'
}

fn is_valid_identifier_char( ch: char) -> bool{
    is_letter(ch) || ch == '_'
}


fn match_keyword(identifier: &Vec<char>, pos: SourceLocation) -> Result<Token,String>{
    let identifier_str : String = identifier.into_iter().collect();
    match &identifier_str[..]{
        "if" => Ok(Token::If(pos)),
        "else" => Ok(Token::Else(pos)),
        "true" => Ok(Token::True(pos)),
        "false" => Ok(Token::False(pos)),
        "node" => Ok(Token::Node(pos)),
        "and" => Ok(Token::AndAlso(pos)),
        "or" => Ok(Token::OrElse(pos)),
        _ => Err(String::from("Unknown keyword"))
    }
}
