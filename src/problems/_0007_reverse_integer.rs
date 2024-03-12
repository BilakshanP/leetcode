pub fn reverse(x: i32) -> i32 {
    let mut x = x as i64;
    let mut result = 0;

    while x != 0 {
        let digit = x % 10;
        x /= 10;
        result = result * 10 + digit;
    }

    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        return 0;
    }

    result as i32
}