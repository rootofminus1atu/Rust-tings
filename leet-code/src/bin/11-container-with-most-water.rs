use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_area = 0;

        while left < right {
            let left_val = height[left];
            let right_val = height[right];

            let current_area = (right - left) as i32 * left_val.min(right_val);

            max_area = max_area.max(current_area);

            match left_val.cmp(&right_val) {
                Ordering::Less => left += 1,
                _ => right -= 1
            }
        }

        max_area
    }
}


fn main() {
    let v = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let area = Solution::max_area(v);

    println!("area: {}", area);
}