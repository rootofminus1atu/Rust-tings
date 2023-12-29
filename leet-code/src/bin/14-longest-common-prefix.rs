#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()], "fl".to_string())]
    #[case(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()], "".to_string())]
    fn test_longest_common_prefix(#[case] words: Vec<String>, #[case] outcome: String) {
        assert_eq!(Solution::longest_common_prefix(words), outcome);
    }
}

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        let mut res = String::new();

        loop {
            let chars = strs.iter()
                .map(|s| s.chars().nth(i))
                .collect::<Vec<_>>();

            if let Some(c) = Solution::coalesce_chars(chars) {
                res.push(c);
            } else {
                break;
            }

            i += 1;
        }

        res
    }

    fn coalesce_chars(chars: Vec<Option<char>>) -> Option<char> {
        match chars.as_slice() {
            [] => None,
            [first_char, rest @ ..] if rest.iter().all(|&c| c == *first_char && c.is_some()) => *first_char,
            _ => None,
        }
    }


    pub fn alt_solution(strs: Vec<String>) -> String {
        strs.into_iter().reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(c1, c2)| c1 == c2)
                .map(|(c, _)| c)
                .collect()
        }).unwrap()
    }
}


fn main() {
    // let words = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];


}