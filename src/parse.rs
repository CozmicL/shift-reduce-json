use crate::unmarshal::JsonValue;
use crate::lexer::lex;
use crate::grammar::{StackElement, ElementType};
use crate::util::{check_prefix_exists, get_value, NOMATCH, PARTIALMATCH};

use crate::action::action;

#[derive(Debug)]
enum ParseError<'a> {
    LexingError,
    ParsingError,
    UnexpectedToken(ElementType<'a>),
}

fn parse(input: &str) -> Result<JsonValue, ParseError> {
    let tokens = match lex(input) {
        Ok(token) => token,
        Err(_) => return Err(ParseError::LexingError),
    };

    let mut stack: Vec<StackElement> = Vec::new();
    let mut reduced_performed = true;

    for mut i in 0..tokens.len() {
        let lookahead = &tokens[i];
        let match_type = check_prefix_exists(stack, *lookahead);

        if match_type != NOMATCH {
            let next_index = i + 1;
            stack.push(StackElement{
                value: Some(lookahead),
                rule: None
            });
            if match_type == PARTIALMATCH {
                i = next_index;
                continue;
            }
        } else if !reduced_performed {
            return Err(ParseError::UnexpectedToken(lookahead.token_type));
        }

        let (json_element, offset) = action(stack); 

        if offset != 0 {
            stack.truncate(stack.len() - offset as usize);
            stack.push(StackElement {
                value: None,
                rule: Some(json_element),
            });
            reduced_performed = true;
        } else {
            reduced_performed = false;
        }
    }

    loop {
        let (json_element, offset) = action(stack); 

        if offset != 0 {
            stack.truncate(stack.len() - offset as usize);
            stack.push(StackElement {
                value: None,
                rule: Some(json_element),
            });
        } else {
            break;
        }
    }

    if stack.len() != 1 {
        return Err(ParseError::ParsingError);
    }

    let val = get_value(stack);

    Ok(JsonValue::new(val, "nil"))
}

