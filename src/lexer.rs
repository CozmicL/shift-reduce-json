use crate::grammar::{LT_ARRAY_END, LT_ARRAY_START, LT_BOOLEAN, LT_COLON, LT_COMMA, LT_DIGITS, LT_EXPONENT, LT_FRACTION_SYMBOL, LT_NULL, LT_OBJECT_END, LT_OBJECT_START, LT_SIGN, LT_STRING};

use crate::grammar::ElementType;
use std::collections::HashMap;

use std::any::Any;
use std::fmt;
#[derive(Debug)]
pub enum TokenError {
    StringLexFailure(String),
    UnrecognizedTokenError
}

pub struct Token<'a> {
    pub value: Box<dyn Any>,
    pub token_type: ElementType<'a>,
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value.downcast_ref::<i32>() {
            write!(f, "Token {{ value: {:?}, token_type: {:?} }}", value, self.token_type)
        } else if let Some(value) = self.value.downcast_ref::<f64>() {
            write!(f, "Token {{ value: {:?}, token_type: {:?} }}", value, self.token_type)
        } else if let Some(value) = self.value.downcast_ref::<String>() {
            write!(f, "Token {{ value: {:?}, token_type: {:?} }}", value, self.token_type)
        } else {
            // If downcasting fails, fallback to a generic debug output.
            write!(f, "Token {{ value: nil, token_type: {:?} }}", self.token_type)
        }
    }
}

fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\t' || ch == b'\n'
}

fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}

fn init_special_symbols<'a>() -> HashMap<u8, ElementType<'a>> {
    let mut special_symbols = HashMap::new();
    special_symbols.insert(b'{', LT_OBJECT_START);
    special_symbols.insert(b'}', LT_OBJECT_END);
    special_symbols.insert(b'[', LT_ARRAY_START);
    special_symbols.insert(b']', LT_ARRAY_END);
    special_symbols.insert(b',', LT_COMMA);
    special_symbols.insert(b':', LT_COLON);
    special_symbols.insert(b'.', LT_FRACTION_SYMBOL);
    special_symbols
}


pub fn lex(input: &str) -> Result<Vec<Token>, TokenError> {
    let special_symbols = init_special_symbols();
    let mut tokens = Vec::new();
    let chars: Vec<u8> = input.bytes().collect();

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        match ch {
            _ if special_symbols.contains_key(&ch) => {
                if let Some(&token_type) = special_symbols.get(&ch) {
                    tokens.push(Token {
                        value: Box::new("nil"),
                        token_type,
                    });
                }
                i += 1;
            }
            b'"' => {
                let (token, offset) = match lex_string(input, i) {
                    Ok((token, offset)) => (token, offset),
                    Err(err) => {
                        panic!("Failed to lex string: {:?}", err);
                    }
                };
                tokens.push(token);
                i += offset;
            }
            b't' => {
                if input[i..i + 4] == *"true" {
                    tokens.push(Token {
                        value: Box::new("true"),
                        token_type: LT_BOOLEAN,
                    });
                    i += 4;
                } else {
                    return Err(TokenError::UnrecognizedTokenError);
                }
            }
            b'f' => {
                if input[i..i + 5] == *"false" {
                    tokens.push(Token {
                        value: Box::new("false"),
                        token_type: LT_BOOLEAN,
                    });
                    i += 5;
                } else {
                    return Err(TokenError::UnrecognizedTokenError);
                }
            }
            b'n' => {
                if input[i..i + 4] == *"null" {
                    tokens.push(Token {
                        value: Box::new("null"),
                        token_type: LT_NULL,
                    });
                    i += 4;
                } else {
                    return Err(TokenError::UnrecognizedTokenError);
                }
            }
            b'e' | b'E' =>{
                tokens.push(Token { value: Box::new("exp"), token_type: LT_EXPONENT });
                i+=1;
            }
            b'+' | b'-' =>{
                tokens.push(Token { value: Box::new(ch), token_type: LT_SIGN });
                i+=1;
            }

            _ if is_digit(ch) => {
                let (token, offset) = lex_digits(input, i);
                tokens.push(token);
                i += offset;
            }

            _ if is_whitespace(ch) => {
                i += 1; // Skip whitespace
            }
            _ => {
                return Err(TokenError::UnrecognizedTokenError);
            }
        }
        
    }

    Ok(tokens)
}

fn lex_digits(input: &str, mut i: usize) -> (Token, usize) {
    let mut str = String::new();
    let bytes = input.as_bytes();

    while i < bytes.len() && is_digit(bytes[i]) {
        str.push(bytes[i] as char);
        i += 1;
    }

    (
        Token {
            token_type: LT_DIGITS,
            value: Box::new(str.clone()),
        },
        str.len(),
    )
}

fn lex_string(input: &str, mut i: usize) -> Result<(Token, usize), TokenError> {
    i += 1; // Move past the opening quote
    let mut sb = String::new();
    let bytes = input.as_bytes();

    while i < bytes.len() && bytes[i] != b'"' {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            // Handle escape sequence
            i += 1;
            match bytes[i] {
                b'"' | b'\\' => sb.push(bytes[i] as char),
                b'n' => sb.push('\n'),
                b't' => sb.push('\t'),
                b'r' => sb.push('\r'),
                _ => {
                    return Err(TokenError::StringLexFailure(format!("Invalid escape sequence at position {}", i)));
                }
            }
        } else {
            sb.push(bytes[i] as char);
        }
        i += 1;
    }

    if i >= bytes.len() || bytes[i] != b'"' {
        return Err(TokenError::StringLexFailure("String is not properly closed".to_string()));
    }

    i += 1; // Move past the closing quote

    Ok((
        Token {
            token_type: LT_STRING,
            value: Box::new(sb),
        },
        i,
    ))
}
