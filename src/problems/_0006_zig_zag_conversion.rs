pub fn convert_1(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let len: usize = num_rows as usize;

    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut is_going_down: bool = true;

    let mut matrix: Vec<Vec<char>> = vec![vec!['\0'; s.len()]; len];

    for ch in s.chars() {
        matrix[row][col] = ch;

        if is_going_down {
            row += 1;

            if row == len - 1 {
                is_going_down = false;
            }
        } else {
            row -= 1;
            col += 1;

            if row == 0 {
                is_going_down = true;
            }
        }
    }

    let mut s: String = String::with_capacity(s.len());

    for &ch in matrix.iter().flatten() {
        if ch != '\0' {
            s.push(ch)
        }
    }

    s
}

pub fn convert_2(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut result: String = String::new();
    let s: &[u8] = s.as_bytes();
    let len: i32 = s.len() as i32;
    let cycle: i32 = 2 * num_rows - 2;
    for i in 0..num_rows {
        let mut j = i;
        while j < len {
            result.push(s[j as usize] as char);
            if i != 0 && i != num_rows - 1 {
                let temp: i32 = j + cycle - 2 * i;
                if temp < len {
                    result.push(s[temp as usize] as char);
                }
            }
            j += cycle;
        }
    }
    result
}