use std::any::Any;
use crate::{lexer::Token, unmarshal::{JsonValue, JsonValueType}};

pub type ElementType<'a> = &'a str;

pub const NUMBER: ElementType = "<number>";
pub const INTEGER: ElementType = "<integer>";
pub const VALUE: ElementType = "<value>";
pub const ARRAY: ElementType = "<array>";
pub const MEMBERS: ElementType = "<object fields>";
pub const MEMBER: ElementType = "<object field>";
pub const ELEMENTS: ElementType = "<array elements>";
pub const ELEMENT: ElementType = "<array element>";
pub const OBJECT: ElementType = "<object>";
pub const BOOLEAN: ElementType = "<boolean>";
pub const EXPONENT: ElementType = "<exponent>";
pub const FRACTION: ElementType = "<fraction>";
// literal tokens
pub const LT_OBJECT_START: ElementType = "{";
pub const LT_OBJECT_END: ElementType = "}";
pub const LT_ARRAY_START: ElementType = "[";
pub const LT_ARRAY_END: ElementType = "]";
pub const LT_COMMA: ElementType = ",";
pub const LT_COLON: ElementType = ":";
pub const LT_FRACTION_SYMBOL: ElementType = ".";
pub const LT_BOOLEAN: ElementType = "<bool_literal>";
pub const LT_EXPONENT: ElementType = "e/E";
pub const LT_DIGITS: ElementType = "[0-9] (digits)";
pub const LT_NULL: ElementType = "<null>";
pub const LT_SIGN: ElementType = "+/-";
pub const LT_STRING: ElementType = "<string_literal>";

#[derive(Debug)]
pub struct GrammarRule<'a> {
    pub lhs: ElementType<'a>,
    pub rhs: &'a [&'a [&'a str]],
    pub to_json: fn(&[StackElement<'a>]) -> JsonValue<'a>,
}

#[derive(Debug)]
pub struct JsonElement<'a> {
    pub value: Box<dyn Any>,
    pub element_type: ElementType<'a>,
}

impl<'a> AsRef<JsonElement<'a>> for JsonElement<'a> {
    fn as_ref(&self) -> &JsonElement<'a> {
        self
    }
}

impl<'a> JsonElement<'a> {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}



#[derive(Debug)]
pub struct StackElement<'a> {
    pub value: Option<&'a Token<'a>>,
    pub rule: Option<JsonElement<'a>>,
}



fn value_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn boolean_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn object_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}


fn members_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn member_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn array_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}
fn elements_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn element_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn number_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn integer_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn fraction_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

fn exponent_rule<'a>(values: &[StackElement]) -> JsonValue<'a> {

}

pub static GRAMMAR: [GrammarRule; 12] = [
    GrammarRule {
        lhs: VALUE,
        rhs: &[
            &[OBJECT],
            &[ARRAY],
            &[NUMBER],
            &[BOOLEAN],
            &[LT_STRING],
            &[LT_NULL],
        ],
        to_json: value_rule,
    },
    GrammarRule {
        lhs: BOOLEAN,
        rhs: &[
            &[LT_BOOLEAN],
        ],
        to_json: boolean_rule,
    },
    GrammarRule {
        lhs: OBJECT,
        rhs: &[
            &[LT_OBJECT_START, LT_OBJECT_END],
            &[LT_OBJECT_START, MEMBERS, LT_OBJECT_END],
        ],
        to_json: object_rule,
    },
    GrammarRule {
        lhs: MEMBERS,
        rhs: &[
            &[MEMBER],
            &[MEMBERS, LT_COMMA, MEMBER],
        ],
        to_json: members_rule,
    },
    GrammarRule {
        lhs: MEMBER,
        rhs: &[
            &[LT_STRING, LT_COLON, VALUE],
        ],
        to_json: member_rule,
    },
    GrammarRule {
        lhs: ARRAY,
        rhs: &[
            &[LT_ARRAY_START, LT_ARRAY_END],
            &[LT_ARRAY_START, ELEMENTS, LT_ARRAY_END],
        ],
        to_json: array_rule,
    },
    GrammarRule {
        lhs: ELEMENTS,
        rhs: &[
            &[ELEMENT],
            &[ELEMENTS, LT_COMMA, ELEMENT],
        ],
        to_json: elements_rule,
    },
    GrammarRule {
        lhs: ELEMENT,
        rhs: &[
            &[VALUE],
        ],
        to_json: element_rule,
    },
    GrammarRule {
        lhs: NUMBER,
        rhs: &[
            &[INTEGER, FRACTION, EXPONENT],
            &[INTEGER, FRACTION],
            &[INTEGER, EXPONENT],
            &[INTEGER],
        ],
        to_json: number_rule,
    },
    GrammarRule {
        lhs: INTEGER,
        rhs: &[
            &[LT_DIGITS],
            &[LT_SIGN, LT_DIGITS],
        ],
        to_json: integer_rule,
    },
    GrammarRule {
        lhs: FRACTION,
        rhs: &[
            &[LT_FRACTION_SYMBOL, LT_DIGITS],
        ],
        to_json: fraction_rule,
    },
    GrammarRule {
        lhs: EXPONENT,
        rhs: &[
            &[LT_EXPONENT, INTEGER],
        ],
        to_json: exponent_rule,
    },
];

