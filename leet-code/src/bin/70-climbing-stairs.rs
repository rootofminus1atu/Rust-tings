use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 2)]
    #[case(3, 3)]
    fn test_climb_stairs(#[case] stairs: i32, #[case] ways: i32) {
        assert_eq!(Solution::climb_stairs(stairs), ways);
    }
}

struct Solution;

impl Solution {

    pub fn climb_stairs(n: i32) -> i32 {
        let mut mem = HashMap::new();

        pub fn climb(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
            match n {
                1 => 1,
                2 => 2,
                _ => {
                    if let Some(&val) = mem.get(&n) {
                        return val
                    }

                    let new_val = climb(n - 1, mem) + climb(n - 2, mem);
                    mem.insert(n, new_val);
                    
                    new_val
                }
            }
        }

        climb(n, &mut mem)
    }
}




fn main() {
    println!("{}", Solution::climb_stairs(45));
}