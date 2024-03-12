pub fn is_valid_1(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for i in s.chars() {
        match i {
            '('|'{'|'[' => stack.push(i),

            right => {
                let left = match right {
                    ')' => '(', '}' => '{', ']' => '[',
                    _ => unreachable!()
                };

                if let Some(counter_part) = stack.pop() {
                    if left != counter_part {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

pub fn is_valid_2(s: String) -> bool {
    if s.len() & 1 == 1 {
        return false;
    }

    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    for i in s.chars() {
        match i {
            '(' | '[' | '{' => stack.push(i),
            _ => match stack.pop() {
                Some('(') if i == ')' => (),
                Some('[') if i == ']' => (),
                Some('{') if i == '}' => (),
                _ => return false,
                }
        }
    }

    stack.is_empty()
}

pub fn is_valid_3(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}'|')'|']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}

pub fn is_valid_4(s: String) -> bool {
    let mut stack = Vec::new();

    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),

            closing => if Some(closing) != stack.pop() { 
                return false 
            }
        }
    }
    stack.is_empty()
}

pub fn is_valid_5(s: String) -> bool {
    let mut stack = Vec::new();

    for i in s.bytes() {
        match (stack.last().copied(), i) {
            (_, b'(')|(_, b'[')|(_, b'{') => stack.push(i),
            (Some(b'('), b')') | (Some(b'['), b']') | (Some(b'{'), b'}') => { stack.pop(); },
            _ => return false,
        }
    }

    stack.is_empty()
}