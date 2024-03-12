pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
    fn inner(s: String, left: usize, right: usize, size: usize) -> Vec<String> {
        if s.len() == size {
            return vec![s];
        }

        let mut result: Vec<String> = vec![];

        if left < size / 2 {
            result.append(&mut inner(s.clone() + "(", left + 1, right, size));
        }

        if right < left {
            result.append(&mut inner(s.clone() + ")", left, right + 1, size));
        }

        result
    }

    inner("".to_string(), 0, 0, n as usize * 2)
}