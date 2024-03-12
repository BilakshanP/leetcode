pub fn three_sum_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let size: usize = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..(size - 2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut j: usize = i + 1;
        let mut k: usize = size - 1;

        while j < k {
            let ni: i32 = nums[i];
            let nj: i32 = nums[j];
            let nk: i32 = nums[k];
            let s: i32 = ni + nj + nk;

            match s.cmp(&0) {
                std::cmp::Ordering::Less => j += 1,
                std::cmp::Ordering::Equal => {
                    result.push(vec![ni, nj, nk]);

                    while j < k && nj == nums[j + 1] {
                        j += 1
                    }
                    while j < k && nk == nums[k - 1] {
                        k -= 1
                    }

                    j += 1;
                    k -= 1;
                },
                std::cmp::Ordering::Greater => k -= 1
            }
        }
    }

    result
}