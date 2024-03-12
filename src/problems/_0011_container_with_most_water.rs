pub fn max_area_1(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;

    let mut area: i32 = 0;
    let mut max_area: i32 = 0;

    while left < right {
        area = (right - left) as i32 * height[left].min(height[right]);
        max_area = max_area.max(area);

        if (height[left] < height[right]) {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}