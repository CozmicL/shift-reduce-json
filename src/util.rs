use crate::grammar::{JsonElement,StackElement,ElementType,GRAMMAR};
use crate::lexer::Token;


#[derive(Debug)]
pub enum ValueError {
    EmptyStack,
    InvalidType,
    InvalidLength,
    MissingRule,
}

type PrefixMatch = u8;


pub const NOMATCH: PrefixMatch = 0;
pub const PARTIALMATCH: PrefixMatch = 1;
pub const FULLMATCH: PrefixMatch = 2;

fn get_values<'a>(stack: Vec<StackElement<'a>>) -> Result<Vec<JsonElement<'a>>, ValueError> {
    if stack.is_empty() {
        return Err(ValueError::EmptyStack);
    }

    if let Some(rule) = &stack[0].rule {
        if let Some(values) = rule.as_ref().as_any().downcast_ref::<Vec<JsonElement<'a>>>() {
            if values.len() != 1 {
                return Err(ValueError::InvalidLength);
            } else {
                return Ok(*values.clone());
            }
        } else {
            return Err(ValueError::InvalidType);
        }
    } else {
        return Err(ValueError::MissingRule);
    }
}

pub fn get_value<'a>(stack: Vec<StackElement<'a>>) -> Result<JsonElement<'a>, ValueError> {
    let values = get_values(stack)?;

    let val = match values.get(0) {
        Some(first_value) => Ok(*first_value.clone()),
        None => Err(ValueError::InvalidLength), 
    };

    val
}


fn stack_to_token<'a>(stack: &[StackElement<'a>]) -> Vec<&'a str> {
    let mut a: Vec<&'a str> = Vec::new();

    for e in stack.iter() {
        if let Some(rule) = &e.rule {
            a.push(rule.element_type); 
        } else if let Some(value) = &e.value {
            a.push(value.token_type); 
        }
    }

    a
}

pub fn check_prefix_exists<'a>(stack: Vec<StackElement<'a>>,lookahead:Token) -> PrefixMatch{
    let mut elems: Vec<ElementType>;

    let stack_size = stack.len();

    if stack_size >= 2 {
        let slice = &stack[stack_size - 2..]; 
        elems.extend_from_slice(&stack_to_token(slice));
    } else if stack_size == 1 {
        let slice = &stack[0..1]; 
        elems.extend_from_slice(&stack_to_token(slice));
    }

    elems.push(lookahead.token_type);

    let size = elems.len();

    for i in (0..size).rev(){
        let match_type = check_prefix(&elems[i..size]); //TODO
        if match_type != NOMATCH{
            return match_type;
        }
    }
    return NOMATCH;
}

struct Payload{
    match_type: u8,
    prod_size: i32,
}

fn check_prefix(candidates:&[&str]) -> PrefixMatch{
    let mut data: Vec<Payload> = Vec::new();

    for rule in &GRAMMAR{
        for production in rule.rhs{
            let csize = candidates.len();
            let rsize = production.len();

            if csize > rsize{
                continue;
            }

            let mut did_not_match = false;

            for i in 0..csize{
                if candidates[i] != production[i]{
                    did_not_match = true;
                    break;
                }
            }

            if did_not_match{
                continue;
            }

            let p:Payload;

            if csize == rsize {
                p = Payload{
                    match_type: FULLMATCH,
                    prod_size: rsize as i32,
                };
            }else{
                p = Payload{
                    match_type: PARTIALMATCH,
                    prod_size: rsize as i32,
                };
            }

            data.push(p);
        }
    }

    if data.len() == 0 {
        return NOMATCH;
    }

    data.sort_by(|a, b| b.prod_size.cmp(&a.prod_size));

    return data[0].match_type;
}