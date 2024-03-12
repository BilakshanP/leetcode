pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = strs[0].as_str();

    for (i, c) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
           
            if let Some(ch) = s.chars().nth(i) {
                if ch != c {
                    return first_str[..i].to_string();
                }
            } else {
                return first_str[..i].to_string();
            }
        }
    }

    first_str.to_string()
}

pub fn longest_common_prefix_2(strs: Vec<String>) -> String { 
    match strs.is_empty() {
        true => "".to_string(),
        _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| acc.chars().zip(x.chars()).take_while(|(x,y)| x == y).map(|(x, _)| x).collect())
    }
}