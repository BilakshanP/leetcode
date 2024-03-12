pub fn length_of_last_word_1(s: String) -> i32 {
    let mut v: Vec<&str> = s.strip_prefix(' ')
                            .unwrap_or(&s)
                            .strip_suffix(' ')
                            .unwrap_or(&s)
                            .split(' ')
                            .collect::<Vec<_>>();

    v.retain(|sstr: &&str| !sstr.is_empty());
    
    v.last().unwrap_or(&"").len() as i32
}

pub fn length_of_last_word_2(s: String) -> i32 {
    let trimmed: &str = s.trim();

    match trimmed.rfind(' ') {
        Some(idx) => (trimmed.len() - idx - 1) as i32,
        None => trimmed.len() as i32,
    }
}


pub fn length_of_last_word_3(s: String) -> i32 {
    s.trim_end()
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .count() as i32
}

pub fn length_of_last_word_5(s: String) -> i32 {
    let mut len: i32 = 0;

    for c in s.trim_end().chars().rev() {
        if c == ' ' { break; }
        len += 1;
    }

    len
}

pub fn length_of_last_word_6(s: String) -> i32 {
    s.split_ascii_whitespace()
     .last()
     .unwrap()
     .len() as i32
}

pub fn length_of_last_word(s: String) -> i32 {
    s.trim()
        .split(' ')
        .last()
        .unwrap()
        .len() as i32
}