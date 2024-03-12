pub fn int_to_roman_1(num: i32) -> String {
    let lut: Vec<(i32, &str)> = vec![
        (1000, "M"), (900, "CM"),
        (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"),
        (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"),
        (5, "V"), (4, "IV"),
        (1, "I"),
    ];

    let mut tmp: i32 = num;
    let mut out: String = String::new();

    for (val, sym) in lut {
        while tmp >= val {
            out.push_str(sym);
            tmp -= val;
        }
    }

    out
}

pub fn int_to_roman_2(num: i32) -> String {
    match num {
        1..=3 => "I".repeat(num as usize),
        4 => "IV".to_string(),
        5..=8 => "V".to_string() + &"I".repeat((num - 5) as usize),
        9 => "IX".to_string(),
        10..=39 => "X".repeat((num / 10) as usize) + &int_to_roman_2(num % 10),
        40..=49 => "XL".to_string() + &int_to_roman_2(num - 40),
        50..=89 => "L".to_string() + &int_to_roman_2(num - 50),
        90..=99 => "XC".to_string() + &int_to_roman_2(num - 90),
        100..=399 => "C".repeat((num / 100) as usize) + &int_to_roman_2(num % 100),
        400..=499 => "CD".to_string() + &int_to_roman_2(num - 400),
        500..=899 => "D".to_string() + &int_to_roman_2(num - 500),
        900..=999 => "CM".to_string() + &int_to_roman_2(num - 900),
        1000..=3999 => "M".repeat((num / 1000) as usize) + &int_to_roman_2(num % 1000),
        _ => "".to_string(),
    }
}