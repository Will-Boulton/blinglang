use super::token::Token;
use super::Lexer;
use super::token::SourceLocation;

pub(crate) const ESCAPE_CHAR: char = '\\';
pub(crate) const CHAR_LITERAL_STARTSTOP: char = '\'';
pub(crate) const STR_LITERAL_STARTSTOP: char = '\"';

#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) enum LiteralLexFailureReason{
    UnclosedLiteral(SourceLocation),
    UnknownEscapeCharacter(SourceLocation,char),
    InvalidCharLiteral(SourceLocation)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) enum Mode{
    Char,   // ' '
    Str     // " "
}

impl Mode {
    fn open_close_char(self) -> char{
        match self{
            Char => CHAR_LITERAL_STARTSTOP,
            Str => '"'
        }
    }
}

pub(crate) trait LiteralMode{
    fn is_literal_mode(self) -> bool;
    fn literal_mode(self) -> Option<Mode>;
}

impl LiteralMode for char{
    fn is_literal_mode(self) -> bool{
        self.literal_mode().is_some()
    }

    fn literal_mode(self) -> Option<Mode>{
        match self{
            CHAR_LITERAL_STARTSTOP => Some(Mode::Char),
            STR_LITERAL_STARTSTOP => Some(Mode::Str),
            _ => None
        }
    }
}

fn escaped_charater(c: char, m: Mode ) -> Option<char> {
    return match c {
        'r' => Some('\r'),
        'n' => Some('\n'),
        '\\' => Some('\\'),
        '\'' if m == Mode::Char => Some('\''),
        '"' if m == Mode::Str => Some('"'),
        _ => None
    }
}

pub(crate) fn lex_literal(l: &mut Lexer<'_>) -> Result<Token,LiteralLexFailureReason>{
    let mode = l.ch.literal_mode().unwrap();
    match mode {
        Mode::Char => lex_character_literal(l),
        Mode::Str => lex_string_literal(l)
    }
}

// lex a string literal
fn lex_string_literal(l: &mut Lexer<'_>) -> Result<Token,LiteralLexFailureReason>{
    let mut literal = Vec::new();
    let mut escape = false;
    loop {
        let next = l.peek_next();
        if next.is_none() {
            return Err(LiteralLexFailureReason::UnclosedLiteral(l.location()))
        }
        l.advance_pos();
        match next.unwrap() {
            STR_LITERAL_STARTSTOP => {
                if escape {
                    literal.push(STR_LITERAL_STARTSTOP);
                    escape = false;
                } else {
                    return Ok(Token::StringLiteral(l.location(), literal.into_iter().collect()))
                }
            }
            ESCAPE_CHAR => {
                if escape {
                    literal.push(ESCAPE_CHAR);
                }
                escape = !escape
            },
            mut c =>{
                if escape {
                    match escaped_charater(c, Mode::Str){
                        Some(a) => { c = a; }
                        None => return Err(LiteralLexFailureReason::UnknownEscapeCharacter(l.location(),c))
                    }
                    escape = false;
                }
                literal.push(c);
            }
        }
    }
}

fn lex_character_literal(l: &mut Lexer<'_>) -> Result<Token,LiteralLexFailureReason>{ 
    let skip_bad_char = |c: char| c != '\'' && !c.is_whitespace();
    let next = l.peek_next();
    l.advance_pos();
    match lex_character_literal_core(l) {
        Ok(token) => match next {
            Some('\'') => return Ok(token),
            Some(_) => {
                l.skip_while(skip_bad_char);
                return Err(LiteralLexFailureReason::UnclosedLiteral(l.location()))
            },
            None => return Err(LiteralLexFailureReason::UnclosedLiteral(l.location()))
        },
        Err(x) => {
            l.skip_while(skip_bad_char);
            return Err(x)
        }
    }   
}

fn lex_character_literal_core(l: &mut Lexer<'_>) -> Result<Token,LiteralLexFailureReason>{ 
    let ch = l.peek_next();
    if ch.is_none(){
        return Err(LiteralLexFailureReason::UnclosedLiteral(l.location()))
    }
    l.advance_pos();
    let ch_ =  ch.unwrap();
    if ch_ == ESCAPE_CHAR {
        let next_ch = l.peek_next();
        return match next_ch {
            Some(x)  => 
                match escaped_charater(x, Mode::Char) {
                    Some(escaped) => {
                        l.advance_pos();
                        Ok(Token::CharLiteral( l.location(),escaped))
                    },
                    _ => Err(LiteralLexFailureReason::UnknownEscapeCharacter(l.location(),x))
                },
            None => Err(LiteralLexFailureReason::UnclosedLiteral(l.location()))
        }
    } else {
        Ok(Token::CharLiteral( l.location(),ch_))
    }
}