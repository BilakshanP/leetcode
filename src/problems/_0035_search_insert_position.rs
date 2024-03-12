pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
    for (n, &i) in nums.iter().enumerate() {
        if i >= target {
            return n as i32;
        }
    }

    nums.len() as i32
}

pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    fn search_recurse(nums: Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
        if left > right {
            return left;
        }

        let mid: i32 = left + (right - left) / 2;
    
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Greater => search_recurse(nums, target, left, mid - 1),
            std::cmp::Ordering::Less => search_recurse(nums, target, mid + 1, right),
        }
    }

    let last_index: i32 = nums.len() as i32 - 1;
    search_recurse(nums, target, 0, last_index)
}

pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(x) | Err(x) => x as i32,
    }
}

pub fn search_insert_4(nums: Vec<i32>, target: i32) -> i32 {
    nums.iter() 
        .take_while(|x| **x < target)
        .count() as i32
}