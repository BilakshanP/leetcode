pub fn min_pair_sum_1(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let len_half: usize = nums.len() / 2;

    let (left, right) = nums.split_at_mut(len_half);

    let mut pair_sum: Vec<i32> = Vec::with_capacity(len_half);

    for (&l, &r) in left.iter().zip(right.iter().rev()) {
        pair_sum.push(l + r);
    }

    pair_sum.iter().fold(0, |acc, &curr| if acc > curr { acc } else { curr })
}

pub fn min_pair_sum_2(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let len_half: usize = nums.len() / 2;

    let (left, right) = nums.split_at_mut(len_half);

    let mut pair_sum: Vec<i32> = Vec::with_capacity(len_half);

    for (&l, &r) in left.iter().zip(right.iter().rev()) {
        pair_sum.push(l + r);
    }

    pair_sum.iter().copied().max().unwrap()
}

pub fn min_pair_sum_3(mut nums: Vec<i32>) -> i32 {
    let len_half: usize = nums.len() / 2;

    nums.sort();

    nums.iter()
        .take(len_half)
        .zip(
            nums.iter()
                .rev()
                .take(len_half)
        )
        .map(|(a, b)| a + b)
        .max()
        .unwrap()
}

