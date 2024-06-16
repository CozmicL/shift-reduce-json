use crate::grammar::{ElementType, JsonElement, StackElement, GRAMMAR};

pub fn action(stack: Vec<StackElement>) -> (JsonElement, i32) {
    let mut je: Option<JsonElement> = None;
    let mut offset: i32 = 0;

    for rule in &GRAMMAR {
        for production in rule.rhs {
            let size = production.len();

            if size > stack.len() {
                continue;
            }

            let actual = top_n_of_stack(&stack, size);
            let matches = compare(production, &actual);

            if matches && size as i32 > offset {
                je = Some(JsonElement {
                    value: rule.to_json(&stack[stack.len() - size..]),
                    element_type: rule.lhs.clone(),
                });
                offset = size as i32;
            }
        }
    }

    (je.unwrap(), offset)
}

fn top_n_of_stack<'a>(stack: &[StackElement], count: usize) -> Vec<ElementType<'a>> {
    let count = count.min(stack.len());
    let slice = &stack[stack.len() - count..];

    let mut elements = Vec::with_capacity(count);

    for el in slice {
        if let Some(token) = &el.value {
            elements.push(token.token_type.clone());
        } else if let Some(rule) = &el.rule {
            elements.push(rule.element_type.clone());
        }
    }

    elements
}

fn compare<T: PartialEq>(expansion: &[T], actual: &[T]) -> bool {
    if expansion.len() != actual.len() {
        return false;
    }

    for i in 0..expansion.len() {
        if expansion[i] != actual[i] {
            return false;
        }
    }

    true
}
