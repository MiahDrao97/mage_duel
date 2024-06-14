use core::str::Chars;
use core::convert::From;
use crate::{game_zones::types::DamageTypeParseError, parsing::tokens::*};

#[derive(Debug)]
pub enum TokenizerError {
    ParseBoolError(core::str::ParseBoolError),
    ParseIntError(std::num::ParseIntError),
    ParseDamageTypeError(DamageTypeParseError),
    InvalidSyntax
}

impl From<core::str::ParseBoolError> for TokenizerError {
    fn from(err: core::str::ParseBoolError) -> Self {
        TokenizerError::ParseBoolError(err)
    }
}

impl From<std::num::ParseIntError> for TokenizerError {
    fn from(err: std::num::ParseIntError) -> Self {
        TokenizerError::ParseIntError(err)
    }
}

impl From<DamageTypeParseError> for TokenizerError {
    fn from(err: DamageTypeParseError) -> Self {
        TokenizerError::ParseDamageTypeError(err)
    }
}

pub fn tokenize(script: &str) -> Result<Vec<Tokens>, TokenizerError> {
    let mut tokens = vec![ ];
    let mut chars = script.chars();
    let mut next: Box<Option<char>> = Box::new(None);
        
    loop {
        let first: char;
        
        if let Some(c) = *next {
            *next.as_mut() = None;
            if c.is_whitespace() {
                continue;
            }
            first = c;
        } else if let Some(c) = consume_white_space(&mut chars) {
            first= c;
        } else {
            break;
        }

        let token = read_next_token(first, &mut chars, &mut next)?;
        if let Tokens::Comment = token {
            // comments won't get pushed: just read until newline or eof
            read_until_newline_or_eof(&mut chars);
        } else {
            tokens.push(token);
        }
    }
    tokens.push(Tokens::EOF);

    Ok(tokens)
}

fn read_next_token(first: char, chars: &mut Chars, next_first: &mut impl AsMut<Option<char>>) -> Result<Tokens, TokenizerError> {
    if first.is_numeric() {
        let numeric_token = parse_numeric(first, chars, next_first)?;
        return Ok(numeric_token);
    } else if first.is_alphabetic() || first == '$' || first == '_' {
        return Ok(parse_identifier_keyword_or_damage_type(first, chars, next_first));
    } else {
        let syntax_token = parse_syntax(first, chars, next_first)?;
        return Ok(syntax_token);
    }
}

fn parse_numeric(first: char, chars: &mut Chars, next_first: &mut impl AsMut<Option<char>>) -> Result<Tokens, std::num::ParseIntError> {
    let mut char_vec = vec![ first ];

    while let Some(next) = chars.next() {
        if next.is_numeric() {
            char_vec.push(next);
        } else {
            *next_first.as_mut() = Some(next);
            break;
        }
    }

    let num_string: String = char_vec.into_iter().collect();
    let int_token = IntToken::try_from(num_string)?;

    Ok(Tokens::Numeric(int_token))
}

fn parse_identifier_keyword_or_damage_type(first: char, chars: &mut Chars, next_first: &mut impl AsMut<Option<char>>) -> Tokens {
    let mut char_vec = vec![ first ];

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '$' || next == '_' {
            char_vec.push(next);
        }
        else {
            *next_first.as_mut() = Some(next);
            break;
        }
    }

    let final_string: String = char_vec.into_iter().collect();
    if SYMBOLS.contains(&final_string.as_str()) {
        return Tokens::Symbol(StringToken::from(final_string));
    }
    else if let Ok(bool_token) = BoolToken::try_from(final_string.clone()) {
        return Tokens::Boolean(bool_token);
    }
    else if let Ok(damage_type_token) = DamageTypeToken::try_from(final_string.clone()) {
        return Tokens::DamageType(damage_type_token);
    }
    Tokens::Identifier(StringToken::from(final_string))
}

fn parse_syntax(first: char, chars: &mut Chars, next_first: &mut impl AsMut<Option<char>>) -> Result<Tokens, TokenizerError> {
    if !SYMBOLS.contains(&first.to_string().as_str()) {
        return Err(TokenizerError::InvalidSyntax);
    } else if first == '=' {
        if let Some(next) = chars.next() {
            if next == '>' {
                let token = StringToken::from("=>");
                return Ok(Tokens::Symbol(token));
            } else if next == '=' {
                let token = StringToken::from("==");
                return Ok(Tokens::Symbol(token));
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    } else if first == '/' {
        if let Some(next) = chars.next() {
            if next == '/' {
                return Ok(Tokens::Comment);
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    } else if first == '+' {
        if let Some(next) = chars.next() {
            if next == '!' {
                let token = StringToken::from("+!");
                return Ok(Tokens::Symbol(token));
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    } else if first == '<' {
        if let Some(next) = chars.next() {
            if next == '=' {
                let token = StringToken::from("<=");
                return Ok(Tokens::Symbol(token));
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    } else if first == '>' {
        if let Some(next) = chars.next() {
            if next == '=' {
                let token = StringToken::from(">=");
                return Ok(Tokens::Symbol(token));
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    } else if first == '~' {
        if let Some(next) = chars.next() {
            if next == '=' {
                let token = StringToken::from("~=");
                return Ok(Tokens::Symbol(token));
            } else {
                *next_first.as_mut() = Some(next);
            }
        }
    }
    
    Ok(Tokens::Symbol(StringToken::from(first.to_string())))
}

fn consume_white_space(chars: &mut Chars) -> Option<char> {
    loop {
        if let Some(next) = chars.next() {
            if !next.is_whitespace() {
                return Some(next);
            }
        } else {
            return None;
        }
    }
}

fn read_until_newline_or_eof(chars: &mut Chars) {
    loop {
        if let Some(next) = chars.next() {
            if next == '\n' {
                break;
            }
        } else {
            break;
        }
    }
}