use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(vec![3, 2, 4], 6, vec![1, 2])]
    #[case(vec![3, 3], 6, vec![0, 1])]
    fn test_two_sum(#[case] nums: Vec<i32>, #[case] target: i32, #[case] outcome: Vec<i32>) {
        assert_eq!(Solution::two_sum(nums, target), outcome);
    }
}

fn main() {
    let tests = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];

    for (nums, target, outcome) in tests.into_iter() {
        assert_eq!(Solution::two_sum(nums, target), outcome);
    }
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut store: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;

            if let Some(&found_idx) = store.get(&diff) {
                return vec![found_idx, i as i32]
            } else {
                store.insert(num, i as i32);
            }
        }

        vec![]
    }

    pub fn naive_sol(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num1) in nums.iter().enumerate() {
            for (j, num2) in nums.iter().enumerate() {
                if i != j {
                    if num1 + num2 == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }
        }

        vec![]
    }
}