pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut prev_val: i32 = 0;

    for c in s.chars() {
        let val: i32 = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        };

        result += if val > prev_val {
            val - 2 * prev_val
        } else {
            val
        };

        prev_val = val;
    }

    result
}