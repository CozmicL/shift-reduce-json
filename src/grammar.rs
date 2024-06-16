use std::any::Any;

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
    pub to_json: fn(&[StackElement<'a>]) -> JsonValue,
}

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
    pub value: Option<&'a str>,
    pub rule: Option<JsonElement<'a>>,
}

impl<'a> StackElement<'a> {
    pub fn new(value: Option<&'a str>, rule: Option<JsonElement<'a>>) -> Self {
        StackElement { value, rule }
    }

    pub fn value(&self) -> &dyn Any {
        self.value.as_ref().unwrap() as &dyn Any
    }

    pub fn as_json_value(&self) -> JsonValue {
        if let Some(rule) = &self.rule {
            match rule.element_type {
                LT_STRING => JsonValue::new(rule.as_any().downcast_ref::<String>().unwrap().clone(), JsonValueType::STRING),
                LT_BOOLEAN => JsonValue::new(rule.as_any().downcast_ref::<bool>().unwrap().clone(), JsonValueType::BOOL),
                LT_NULL => JsonValue::new("null".to_string(), JsonValueType::NULL),
                OBJECT => JsonValue::new(rule.as_any().downcast_ref::<Vec<(&str, JsonValue)>>().unwrap().clone(), JsonValueType::OBJECT),
                ARRAY => JsonValue::new(rule.as_any().downcast_ref::<Vec<JsonValue>>().unwrap().clone(), JsonValueType::ARRAY),
                _ => JsonValue::new("".to_string(), JsonValueType::STRING), // Handle other types appropriately
            }
        } else {
            JsonValue::new(self.value.unwrap().to_string(), JsonValueType::STRING)
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub value: &'a str,
}

impl<'a> Token<'a> {
    pub fn value(&self) -> &str {
        self.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub enum JsonValueType {
    STRING,
    NULL,
    BOOL,
    OBJECT,
    ARRAY,
    NUMBER,
}

#[derive(Debug)]
pub struct JsonValue {
    pub value: Box<dyn Any>,
    pub value_type: JsonValueType,
}

impl JsonValue {
    pub fn new<T: 'static>(value: T, element_type: JsonValueType) -> Self {
        JsonValue {
            value: Box::new(value),
            value_type: element_type,
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        &*self.value
    }
}

fn value_rule(values: &[StackElement]) -> JsonValue {
    let v = values[0].value();
    if let Some(str_val) = v.downcast_ref::<&str>() {
        return JsonValue::new(str_val.to_string(), JsonValueType::STRING);
    } else if v.downcast_ref::<()>().is_some() {
        return JsonValue::new("null".to_string(), JsonValueType::NULL);
    }
    values[0].as_json_value()
}

fn boolean_rule(values: &[StackElement]) -> JsonValue {
    let b = values[0].value().downcast_ref::<&str>().unwrap() == &"true";
    JsonValue::new(b, JsonValueType::BOOL)
}

fn object_rule(values: &[StackElement]) -> JsonValue {
    if values.len() == 2 {
        JsonValue::new::<Vec<(&str, JsonValue)>>(vec![], JsonValueType::OBJECT)
    } else {
        values[1].as_json_value()
    }
}


fn members_rule(values: &[StackElement]) -> JsonValue {
    let size = values.len();
    let mut members = vec![];
    let member = values[size - 1].as_json_value().as_any().downcast_ref::<Vec<(&str, JsonValue)>>().unwrap().clone();

    if size == 3 {
        members = *values[0].as_json_value().as_any().downcast_ref::<Vec<(&str, JsonValue)>>().unwrap().clone();
    }

    members.extend(*member);

    JsonValue::new(members, JsonValueType::OBJECT)
}

fn member_rule(values: &[StackElement]) -> JsonValue {
    let key = values[0].value().downcast_ref::<&str>().unwrap();
    let value_obj = values[2].as_json_value();

    JsonValue::new(vec![(key, value_obj)], JsonValueType::OBJECT)
}

fn array_rule(values: &[StackElement]) -> JsonValue {
    if values.len() == 2 {
        JsonValue::new::<Vec<JsonValue>>(vec![], JsonValueType::ARRAY)
    } else {
        values[1].as_json_value()
    }
}
fn elements_rule(values: &[StackElement]) -> JsonValue {
    let size = values.len();
    let mut elements = vec![];
    if size == 3 {
        elements = *values[0].as_json_value().as_any().downcast_ref::<Vec<JsonValue>>().unwrap().clone();
    }

    let element = values[size - 1].as_json_value();
    elements.push(element);

    JsonValue::new(elements, JsonValueType::ARRAY)
}

fn element_rule(values: &[StackElement]) -> JsonValue {
    values[0].as_json_value()
}

fn number_rule(values: &[StackElement]) -> JsonValue {
    let size = values.len();
    let integer_value = values[0].as_json_value().as_any().downcast_ref::<String>().unwrap();

    let fraction = if size >= 2 && values[1].as_json_value().as_any().downcast_ref::<String>().unwrap().starts_with('.') {
        values[1].as_json_value().as_any().downcast_ref::<String>().unwrap().to_string()
    } else {
        "".to_string()
    };

    let exponent = if size == 2 && values[1].as_json_value().as_any().downcast_ref::<String>().unwrap().starts_with('e') {
        values[1].as_json_value().as_any().downcast_ref::<String>().unwrap().to_string()
    } else if size == 3 && values[2].as_json_value().as_any().downcast_ref::<String>().unwrap().starts_with('e') {
        values[2].as_json_value().as_any().downcast_ref::<String>().unwrap().to_string()
    } else {
        "".to_string()
    };

    let expression = format!("{}{}{}", integer_value, fraction, exponent);
    let value = expression.parse::<f64>().unwrap_or(0.0); // Handle parse error

    JsonValue::new(value, JsonValueType::NUMBER)
}

fn integer_rule(values: &[StackElement]) -> JsonValue {
    let size = values.len();
    let digits = values[size - 1].value().downcast_ref::<&str>().unwrap();
    let sign = if size == 2 { values[0].value().downcast_ref::<&str>().unwrap() } else { "+" };

    let v = format!("{}{}", sign, digits);
    JsonValue::new(v, JsonValueType::NUMBER)
}

fn fraction_rule(values: &[StackElement]) -> JsonValue {
    let fraction_digits = format!(".{}", values[1].value().downcast_ref::<&str>().unwrap());

    JsonValue::new(fraction_digits, JsonValueType::NUMBER)
}

fn exponent_rule(values: &[StackElement]) -> JsonValue {
    let exponent_expr = format!("e{}", values[1].as_json_value().as_any().downcast_ref::<String>().unwrap());

    JsonValue::new(exponent_expr, JsonValueType::NUMBER)
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

