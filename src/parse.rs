use crate::unmarshal::JsonValue;
use crate::lexer::lex;
use crate::grammar::{StackElement,ElementType};
use crate::util::{check_prefix_exists,get_value, NOMATCH, PARTIALMATCH};

#[derive(Debug)]
enum ParseError<'a> {
    LexingError,
    ParsingError,
    UnexpectedToken(ElementType<'a>)
}



fn parse(input: &str) -> Result<JsonValue,ParseError> {
    let tokens = match lex(input) {
        Ok(token) => token,
        Err(e) => return Err(ParseError::LexingError)
    };

    let stack : Vec<StackElement>;

    let size = tokens.len();

    let reduced_performed = true;

    for i in 0..size{
        let lookahead = &tokens[i];
        let match_type = check_prefix_exists(stack,*lookahead);
        
        if match_type != NOMATCH{
           i+=1;
           stack.push(StackElement::new(Some(*lookahead), None));
           if match_type == PARTIALMATCH{
                continue;
           }
        } else if !reduced_performed{
            return Err(ParseError::UnexpectedToken(lookahead.token_type));
        }

        let (jsonElement,offset) = action(stack);// TODO

        if offset != 0 {
            stack.truncate(stack.len() - offset);
            stack.push(StackElement{
                value: None,
                rule: jsonElement,
            });
            reduced_performed = true;
        }else {
            reduced_performed = false;
        }
    }

    loop{
        let (jsonElement,offset) = action(stack);// TODO

        if offset != 0 {
            stack.truncate(stack.len() - offset);
            stack.push(StackElement{
                value: None,
                rule: jsonElement,
            });
        }else {
            break;
        }
    }

    if stack.len() != 1{
        return Err(ParseError::ParsingError);
    }

    let val = get_value(stack);

    

    Ok(JsonValue::new(val, "nil"))
}