pub fn add_binary_1(mut a: String, mut b: String) -> String {
    fn pad(s1: &mut String, s2: &mut String) {
        let len1: usize = s1.len();
        let len2: usize = s2.len();

        match len1.cmp(&len2) {
            std::cmp::Ordering::Less => {
                s2.insert(0, '0');
                s1.insert_str(0, &"0".repeat((len2 - len1) + 1));
            },
            std::cmp::Ordering::Greater => {
                s1.insert(0, '0');
                s2.insert_str(0, &"0".repeat((len1 - len2) + 1));
            },
            _ => {}
        }
    }
    
    if a == "0" { return b; }
    if b == "0" { return a; }

    pad(&mut a, &mut b);

    let len: usize = a.len();

    let mut a: std::iter::Rev<std::str::Bytes< '_>> = a.bytes().rev();
    let mut b: std::iter::Rev<std::str::Bytes<'_>> = b.bytes().rev();

    let mut output: String = String::with_capacity(len);

    let mut carry: u8 = 0;

    for _ in 0..len {
        output.push(
            match (a.next().unwrap(), b.next().unwrap(), carry) {
                (b'0', b'0', 0) => '0',
                (b'1', b'1', 1) => { carry = 1; '1' },
                (b'0', b'0', 1)|(b'0', b'1', 0)|(b'1', b'0', 0) => { carry = 0; '1' },
                (b'0', b'1', 1)|(b'1', b'0', 1)|(b'1', b'1', 0) => { carry = 1; '0' },
                _ => unreachable!()
            }
        )
    }

    if carry == 1 {
        output.push('1')
    }

    output.chars().rev().collect::<String>().trim_start_matches('0').to_string()
}

pub fn add_binary_2(mut a: String, mut b: String) -> String {
    fn pad(s1: &mut String, s2: &mut String) -> usize {
        let len1: usize = s1.len();
        let len2: usize = s2.len();

        match len1.cmp(&len2) {
            std::cmp::Ordering::Less => {
                s2.push('0');
                s1.push_str(&"0".repeat((len2 - len1) + 1));
                len2
            },
            std::cmp::Ordering::Greater => {
                s1.push('0');
                s2.push_str(&"0".repeat((len1 - len2) + 1));
                len1
            },
            _ => len1
        }
    }

    if a == "0" { return b; }
    if b == "0" { return a; }

    a = a.chars().rev().collect::<String>();
    b = b.chars().rev().collect::<String>();

    let len: usize = pad(&mut a, &mut b);

    let mut a: std::str::Bytes<'_> = a.bytes();
    let mut b: std::str::Bytes<'_> = b.bytes();

    let mut output: String = String::with_capacity(len);

    let mut carry: u8 = 0;

    for _ in 0..len {
        output.push(
            match (a.next().unwrap(), b.next().unwrap(), carry) {
                (b'0', b'0', 0) => '0',
                (b'1', b'1', 1) => { carry = 1; '1' },
                (b'0', b'0', 1)|(b'0', b'1', 0)|(b'1', b'0', 0) => { carry = 0; '1' },
                (b'0', b'1', 1)|(b'1', b'0', 1)|(b'1', b'1', 0) => { carry = 1; '0' },
                _ => unreachable!()
            }
        )
    }

    if carry == 1 {
        output.push('1')
    }

    output.trim_end_matches('0').chars().rev().collect()
}

pub fn add_binary_3(a: String, b: String) -> String {
    fn to_value(c: Option<char>) -> u8 {
        match c {
            None => 0,
            Some('0') => 0,
            Some('1') => 1,
            _ => unreachable!()
        }
    }

    if a.is_empty() { return b; }
    if b.is_empty() { return a; }

    let mut a_chars: std::iter::Rev<std::str::Chars<'_>> = a.chars().rev();
    let mut b_chars: std::iter::Rev<std::str::Chars<'_>> = b.chars().rev();

    let mut next_pair = || {
        match (a_chars.next(), b_chars.next()) {
            (None, None) => None,
            (a, b) => Some((to_value(a), to_value(b))),
        }
    };

    let mut sum: u8;
    let mut carry: bool = false;
    let mut result: String = String::new();

    while let Some((a, b)) = next_pair() {
        sum = if carry {
            a + b + 1
        } else {
            a + b
        };
    
        carry = sum > 1;

        match sum {
            0 | 2 => result.push('0'),
            1 | 3 => result.push('1'),
            _ => unreachable!(),
        }
    }

    if carry {
        result.push('1');
    }

    result
        .chars()
        .rev()
        .collect::<String>()
}

pub fn add_binary_4(a: String, b: String) -> String {
    let mut a: &String = &a;
    let mut b: &String = &b;

    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }

    let res: (String, u128) = a
        .as_bytes()
        .rchunks(127)
        .zip(
            b
            .as_bytes()
            .rchunks(127)
            .chain(std::iter::repeat(&[b'0'; 127][..]))
        )
        .fold((String::new(), 0), |s: (String, u128), (a, b)| {
            let sum: u128 = unsafe {
                s.1
                + u128::from_str_radix(std::str::from_utf8_unchecked(a), 2)
                    .unwrap_or(0)
                + u128::from_str_radix(std::str::from_utf8_unchecked(b), 2)
                    .unwrap_or(0)};
                
                (format!("{:0127b}", sum & 0x7fffffffffffffffffffffffffffffff) + &s.0, sum >> 127)
            }
        );
    
    if res.1 == 1 {
        "1".to_string() + &res.0
    } else {
        let str: String = res.0.trim_start_matches('0').to_string();

        if !str.is_empty() {
            str
        }
        else {
            "0".to_string()
        }
    }
}

pub fn add_binary_5(a: String, b: String) -> String {
    let len1: usize = a.len();
    let len2: usize = b.len();

    let len: usize = len1.max(len2);

    let mut carry: bool = false;
    let mut ans: String = String::new();

    let a: String = format!("{:0>width$}", a, width=len);
    let b: String = format!("{:0>width$}", b, width=len);

    for i in (0..len).rev() {
        match (a.as_bytes()[i] as char, b.as_bytes()[i] as char, carry) {
            ('1', '1', false) | ('1', '0', true) | ('0', '1', true) => { ans.push('0'); carry = true; }
            ('1', '0', false) | ('0', '1', false) | ('0', '0', true) => { ans.push('1'); carry = false; }
            ('0', '0', false) => { ans.push('0'); }
            ('1', '1', true) => { ans.push('1'); }
            _ => unreachable!(),
        }
    }
        if carry {
            ans.push('1');
        }

        ans.chars().rev().collect()
}