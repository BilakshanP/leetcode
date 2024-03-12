pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    pub fn k_sum(k: i32, mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut left: usize = 0;
            let mut right: usize = nums.len() - 1;
            let mut result: Vec<Vec<i32>> = Vec::new();

            while left < right {
                let l_value: i32 = nums[left];
                let r_value: i32 = nums[right];

                if left != 0 && l_value == nums[left - 1] {
                    left += 1;
                    continue;
                }
                if right != nums.len() - 1 && r_value == nums[right + 1] {
                    right -= 1;
                    continue;
                }
                match (l_value + r_value - target).signum() {
                    1 => right -= 1,
                    0 => {
                        result.push([l_value, r_value].to_vec());
                        left += 1
                    }
                    _ => left += 1,
                }
            }
            result
        }

        if nums.len() < k as usize {
            return Vec::new();
        }
        nums.sort();

        match k {
            _ if k != 2 => {
                let len: usize = nums.len() - 1;
                let mut result: Vec<Vec<i32>> = Vec::new();
                for i in 0..=len - 2 {
                    if i != 0 && nums[i - 1] == nums[i] {
                        continue;
                    }
                    if target / k >= nums[i] && target / k <= nums[len] {
                        let t = target.saturating_add(-nums[i]);
                        let k_1_sum: Vec<Vec<i32>> = k_sum(k - 1, nums[i + 1..].to_vec(), t);
                        if !k_1_sum.is_empty() {
                            result.append(
                                &mut k_1_sum
                                    .into_iter()
                                    .map(|mut s| {
                                        s.push(nums[i]);
                                        s
                                    })
                                    .collect::<Vec<Vec<i32>>>(),
                            );
                        }
                    }
                }
                result
            }

            2 => two_sum(nums, target),
            _ => Vec::new(),
        }
    }

    k_sum(4, nums, target)
}