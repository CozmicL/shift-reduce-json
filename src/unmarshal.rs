use std::fmt;

pub type JsonValueType<'a> = &'a str;

pub const STRING: JsonValueType = "STRING";
pub const NUMBER: JsonValueType = "NUMBER";
pub const BOOL: JsonValueType = "BOOLEAN";
pub const NULL: JsonValueType = "NULL";
pub const OBJECT: JsonValueType = "OBJECT";
pub const ARRAY: JsonValueType = "ARRAY";

#[derive(Debug)]
pub struct JsonValue<'a> {
    value: Box<dyn std::any::Any>,
    value_type: JsonValueType<'a>,
}

impl<'a> JsonValue<'a> {
    pub fn new<T: 'static>(value: T, value_type: JsonValueType<'a>) -> Self {
        JsonValue {
            value: Box::new(value),
            value_type,
        }
    }

    pub fn as_any(&self) -> &dyn std::any::Any {
        &*self.value
    }
}

impl<'a> fmt::Display for JsonValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value_type {
            "NULL" => write!(f, "null"),
            "BOOLEAN" => {
                if let Some(v) = self.as_any().downcast_ref::<bool>() {
                    write!(f, "{}", v)
                } else {
                    write!(f, "Invalid bool value")
                }
            }
            "NUMBER" => {
                if let Some(v) = self.as_any().downcast_ref::<f64>() {
                    write!(f, "{}", v)
                } else {
                    write!(f, "Invalid number value")
                }
            }
            "STRING" => {
                if let Some(v) = self.as_any().downcast_ref::<String>() {
                    write!(f, "{}", v)
                } else {
                    write!(f, "Invalid string value")
                }
            }
            "ARRAY" => write!(f, "Array"), // Further implementation needed for arrays
            "OBJECT" => write!(f, "Object"), // Further implementation needed for objects
            _ => write!(f, "Unknown type"),
        }
    }
}
