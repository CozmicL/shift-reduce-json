use std::any::Any;
use crate::lexer::Token;

pub type ElementType<'a> = &'a str;

pub const NUMBER:ElementType = "<number>";
pub const INTEGER:ElementType = "<integer>";
pub const VALUE:ElementType = "<value>";
pub const ARRAY:ElementType = "<array>";
pub const MEMBERS:ElementType = "<object fields>";
pub const MEMBER:ElementType = "<object field>";
pub const ELEMENTS:ElementType = "<array elements>";
pub const ELEMENT:ElementType = "<array element>";
pub const OBJECT:ElementType = "<object>";
pub const BOOLEAN:ElementType = "<boolean>";
pub const EXPONENT:ElementType = "<exponent>";
pub const FRACTION:ElementType = "<fraction>";
 //literal tokens
pub const LT_OBJECT_START:ElementType = "{";
pub const LT_OBJECT_END:ElementType = "}";
pub const LT_ARRAY_START:ElementType = "[";
pub const LT_ARRAY_END:ElementType = "]";
pub const LT_COMMA:ElementType = ",";
pub const LT_COLON:ElementType = ":";
pub const LT_FRACTION_SYMBOL:ElementType = ".";
pub const LT_BOOLEAN:ElementType = "<bool_literal>";
pub const LT_EXPONENT:ElementType = "e/E";
pub const LT_DIGITS:ElementType = "[0-9] (digits)";
pub const LT_NULL:ElementType = "<null>";
pub const LT_SIGN:ElementType = "+/-";
pub const LT_STRING:ElementType = "<string_literal>";

pub struct GrammarRule<'a> {
    pub lhs: ElementType<'a>,
    pub rhs: &'a [&'a [&'a str]],
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
    },
    GrammarRule {
        lhs: BOOLEAN,
        rhs: &[
            &[LT_BOOLEAN],
        ],
    },
    GrammarRule {
        lhs: OBJECT,
        rhs: &[
            &[LT_OBJECT_START, LT_OBJECT_END],
            &[LT_OBJECT_START, MEMBERS, LT_OBJECT_END],
        ],
    },
    GrammarRule {
        lhs: MEMBERS,
        rhs: &[
            &[MEMBER],
            &[MEMBERS, LT_COMMA, MEMBER],
        ],
    },
    GrammarRule {
        lhs: MEMBER,
        rhs: &[
            &[LT_STRING, LT_COLON, VALUE],
        ],
    },
    GrammarRule {
        lhs: ARRAY,
        rhs: &[
            &[LT_ARRAY_START, LT_ARRAY_END],
            &[LT_ARRAY_START, ELEMENTS, LT_ARRAY_END],
        ],
    },
    GrammarRule {
        lhs: ELEMENTS,
        rhs: &[
            &[ELEMENT],
            &[ELEMENTS, LT_COMMA, ELEMENT],
        ],
    },
    GrammarRule {
        lhs: ELEMENT,
        rhs: &[
            &[VALUE],
        ],
    },
    GrammarRule {
        lhs: NUMBER,
        rhs: &[
            &[INTEGER, FRACTION, EXPONENT],
            &[INTEGER, FRACTION],
            &[INTEGER, EXPONENT],
            &[INTEGER],
        ],
    },
    GrammarRule {
        lhs: INTEGER,
        rhs: &[
            &[LT_DIGITS],
            &[LT_SIGN, LT_DIGITS],
        ],
    },
    GrammarRule {
        lhs: FRACTION,
        rhs: &[
            &[LT_FRACTION_SYMBOL, LT_DIGITS],
        ],
    },
    GrammarRule {
        lhs: EXPONENT,
        rhs: &[
            &[LT_EXPONENT, INTEGER],
        ],
    },
];

#[derive(Debug)]
pub struct JsonElement<'a> {
    pub value: Box<dyn Any>,
    pub element_type: ElementType<'a>,
}

impl<'a> JsonElement<'a> {
    pub fn new<T: 'static>(value: T, element_type: ElementType<'a>) -> Self {
        JsonElement {
            value: Box::new(value),
            element_type,
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        &*self.value
    }
}

#[derive(Debug)]
pub struct StackElement<'a> {
    pub value: Option<Token<'a>>,
    pub rule: Option<JsonElement<'a>>,
}

impl<'a> StackElement<'a> {
    pub fn new(value: Option<Token<'a>>, rule: Option<JsonElement<'a>>) -> Self {
        StackElement { value, rule }
    }
}