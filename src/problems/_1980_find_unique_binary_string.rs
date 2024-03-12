pub fn find_different_binary_string_1(nums: Vec<String>) -> String {
    let mut out_string: String = String::with_capacity(nums.len());

    for (n, i) in nums.iter().enumerate() {
      out_string.push(
        match i.chars().nth(n).unwrap() {
            '0' => '1',
            '1' => '0',
            _ => unreachable!()
        }
      )
    }

    out_string
}

pub fn find_different_binary_string_2(nums: Vec<String>) -> String {
    nums
        .iter()
        .enumerate()
        .map(|(i, s)| if s.as_bytes()[i] == b'0' { '1' } else { '0' })
        .collect()
}