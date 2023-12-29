use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // checking rows
        for row in board.iter() {
            let mut seen = HashSet::new();
            let has_duplicates = row.iter()
                .filter(|elem| elem != &&'.')
                .any(|elem| !seen.insert(elem));

            if has_duplicates { return false; }
        }

        // checking cols
        for i in 0..board.len() {
            let mut seen = HashSet::<char>::new();

            for j in 0..board[0].len() {
                let elem = board[j][i];
                if elem == '.' { continue; }

                let found_duplicate = !seen.insert(elem);
                if found_duplicate { return false; }
            }
        }

        // checking 3x3 squares
        for i in 0..3 {
            for j in 0..3 {
                let mut seen = HashSet::<char>::new();

                for n in 0..3 {
                    for m in 0..3 {
                        let elem = board[3 * i + n][3 * j + m];
                        if elem == '.' { continue; }

                        let found_duplicate = !seen.insert(elem);
                        if found_duplicate { return false; }
                    }
                }
            }
        }

        true
    }
}

fn main () {


    let board = 
    vec![vec!['5','3','.','.','7','.','.','.','.']
    ,vec!['6','.','.','1','9','5','.','.','.']
    ,vec!['.','9','8','.','.','.','.','6','.']
    ,vec!['8','.','.','.','6','.','.','.','3']
    ,vec!['4','.','.','8','.','3','.','.','1']
    ,vec!['7','.','.','.','2','.','.','.','6']
    ,vec!['.','6','.','.','.','.','2','8','.']
    ,vec!['.','.','.','4','1','9','.','.','5']
    ,vec!['.','.','.','.','8','.','.','7','9']];

    let yes_no = Solution::is_valid_sudoku(board);
    println!("uhh {}", yes_no);

}