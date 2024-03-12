fn plus_one_1(mut digits: Vec<i32>) -> Vec<i32> {
    fn increment_array(arr: &mut Vec<i32>) -> Vec<i32> {
        if *arr.last().unwrap() == 9 {
            arr.pop();
            let mut new_arr: Vec<i32> = increment_array(arr);
            new_arr.push(0);
            return new_arr;
        } else {
            *arr.last_mut().unwrap() += 1;
        }

        arr.clone()
    }

    let flag: bool = *digits.last().unwrap() == 9;

    if flag {
        digits.insert(0, 0);
    }

    digits = increment_array(&mut digits);

    if flag && digits[0] == 0 {
        digits.remove(0);
    }

    digits
}

pub fn plus_one_2(mut digits: Vec<i32>) -> Vec<i32> {
    for x in digits.iter_mut().rev() {
        match *x == 9 {
            true => *x = 0,
            false => {
                *x += 1;
                return digits;
            }
        }
    }

    digits.insert(0, 1);
    digits
}




pub fn plus_one_3(mut digits: Vec<i32>) -> Vec<i32> {
    fn increment_recursive(digits: &mut [i32]) {
        let last_digit: &mut i32 = digits.last_mut().unwrap();

        match *last_digit {
            9 => {
                *last_digit = 0;
                let len: usize = digits.len();
                increment_recursive(&mut digits[..len - 1])
            },
            _ => *last_digit += 1,
        }
    }

    if digits.iter().all(|d| *d == 9) {
        digits.push(0);
        digits.fill(0);
        *digits.first_mut().unwrap() = 1;
        return digits;
    }

    increment_recursive(&mut digits[..]);

    digits
}