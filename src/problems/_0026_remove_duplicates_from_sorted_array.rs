pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    let mut j: usize = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j as i32
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    let mut j: usize = 0;
    let mut previous: Option<i32> = None;

    for i in 0..nums.len() {
        if Some(nums[i]) != previous {
            previous = Some(nums[i]);
            nums.swap(i, j);
            j += 1;
        }
    }

    j as i32
}

