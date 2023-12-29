#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("III", 3)]
    #[case("LVIII", 58)]
    #[case("MCMXCIV", 1994)]
    fn test_roman_to_int(#[case] roman: String, #[case] numeral: i32) {
        assert_eq!(Solution::roman_to_int(roman), numeral);
    }
}
pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let res: i32 = s.chars()
            .enumerate()
            .map(|(i, c)| {
                let c_next = s.chars().nth(i + 1);

                let c_val = Solution::roman_matcher(c);
                let c_next_val = c_next.map(Solution::roman_matcher).unwrap_or(0);

                if c_val >= c_next_val {
                    c_val
                } else {
                    -c_val
                }
            })
            .sum();

        return res;
    }

    fn roman_matcher(c: char) -> i32 {
        match c {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            'I' => 1,
            _ => 0
        }
    }
}


fn main() {
    
}
