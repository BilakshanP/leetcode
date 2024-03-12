pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (m, i) in nums.iter().enumerate() {
        for (n, j) in nums.iter().enumerate() {
            if m != n && i + j == target {
                return vec![m as i32, n as i32]
            }
        }
    }

    unreachable!()
}

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter()
                                        .enumerate()
                                        .collect::<Vec<(usize, i32)>>();

    nums.sort_unstable_by_key(|&(_, n)| n);
        
    for (k, (i, ni)) in nums.iter().enumerate() {
        if let Ok(jj) = nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) {
            return vec![*i as i32, nums[jj+k+1].0 as i32]
        }
    }

    unreachable!()
}