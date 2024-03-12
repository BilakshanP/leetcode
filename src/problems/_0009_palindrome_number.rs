pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed: i32 = 0;
    let mut remaining: i32 = x;

    while remaining != 0 {
        reversed = reversed * 10 + remaining % 10;
        remaining /= 10;
    }

    x == reversed
}
