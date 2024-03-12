pub fn find_median_sorted_arrays_1(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    nums1.extend(nums2);
    nums1.sort();

    let n: usize = nums1.len();

    if n % 2 == 1 {
        return nums1[(n + 1)/2 - 1] as f64;
    }

    (nums1[n/2 - 1] + nums1[n/2]) as f64 / 2.0
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut c_vec: Vec<i32> = [nums1, nums2].concat();
    c_vec.sort();

    let c_len: usize = (c_vec.len() - 1) >> 1;

    (c_vec[c_len] + c_vec[c_len + ((c_vec.len() - 1) & 1)]) as f64 / 2.0
}