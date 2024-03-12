pub fn my_atoi_1(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut sign: i32 = 1;
    let mut i: usize = 0;

    while i < s.len() && s.as_bytes()[i] == b' ' {
        i += 1;
    }

    if i < s.len() && (s.as_bytes()[i] == b'-' || s.as_bytes()[i] == b'+') {
        sign = if s.as_bytes()[i] == b'-' { -1 } else { 1 };
        i += 1;
    }

    while i < s.len() && s.as_bytes()[i] >= b'0' && s.as_bytes()[i] <= b'9' {
        let digit: i32 = (s.as_bytes()[i] - b'0') as i32;

        if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }

        result = result * 10 + digit;
        i += 1;
    }

    result * sign
}