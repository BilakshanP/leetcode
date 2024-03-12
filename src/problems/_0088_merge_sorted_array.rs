pub fn merge_1(nums1: &mut Vec<i32>, _m: i32, nums2: &mut Vec<i32>, n: i32) {
    *nums1 = nums1.strip_suffix(&[0].repeat(n as usize)[..]).unwrap().to_vec();
    nums1.append(nums2);
    nums1.sort();
}

pub fn merge_2(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    nums1.iter_mut()
        .skip(m as usize)
        .zip(nums2.iter())
        .for_each(|(a, b)| *a = *b);

    nums1.sort()
}