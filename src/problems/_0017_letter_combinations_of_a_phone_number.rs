pub fn letter_combinations_1(digits: String) -> Vec<String> {
    pub fn inner(s: Vec<String>, n: usize, lut: Vec<&str>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        for i in s {
            for ch in lut[n].chars() {
                out.push(i.clone() + &ch.to_string());
            }
        }

        out
    }

    let lut: Vec<&str> = vec![
        "", "", "abc",  "def",
        "ghi",  "jkl",  "mno",
        "pqrs", "tuv", "wxyz"
    ];

    let digits: Vec<usize> = digits.chars().map(|ch| ch as usize - b'0' as usize).collect();

    let mut out: Vec<String> = Vec::new();

    for i in digits {
        out = if out.is_empty() {
            lut[i].chars().map(|ch: char| ch.to_string()).collect()
        } else {
            inner(out, i, lut.clone())
        };
    }

    out
}

pub fn letter_combinations_2(digits: String) -> Vec<String> {
    fn backtrack(i: usize, current: String, result: &mut Vec<String>, digits: &Vec<usize>, lut: &Vec<&str>) {
        if current.len() == digits.len() {
            result.push(current);
            return;
        }
    
        let letters: &str = lut[digits[i]];
    
        for ch in letters.chars() {
            let mut append_str: String = current.clone();
            append_str.push(ch);
            backtrack(i + 1, append_str, result, digits, lut);
        }
    }

    let digits: Vec<usize> = digits.chars().map(|n| n as usize - b'0' as usize).collect::<Vec<usize>>();

    let mut result: Vec<String> = vec![];
    let lut: Vec<&str> = vec![
        "", "", "abc",  "def",
        "ghi",  "jkl",  "mno",
        "pqrs", "tuv", "wxyz"
    ];

    if !digits.is_empty() {
        backtrack(0, "".to_string(), &mut result, &digits, &lut);
    }

    result        
}

