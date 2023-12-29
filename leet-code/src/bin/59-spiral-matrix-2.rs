#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![1,2,3],vec![8,9,4],vec![7,6,5]])]
    #[case(1, vec![vec![1]])]
    fn test_generate_matrix(#[case] dim: i32, #[case] matrix: Vec<Vec<i32>>) {
        assert_eq!(Solution::generate_matrix(dim), matrix);
    }
}

pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        MatrixGenerator::new(n).spiral()
    }

    pub fn alternate_solution(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut matrix = vec![vec![0; n]; n];
        
        let (mut top, mut right, mut bottom, mut left) = (0, n, n, 0);
        let mut k = 1;

        while k <= (n as i32).pow(2) {
            for j in left..right {
                matrix[top][j] = k;
                k += 1;
            }
            top += 1;

            for i in top..bottom {
                matrix[i][right - 1] = k;
                k += 1;
            }
            right -= 1;

            for j in (left..right).rev() {
                matrix[bottom - 1][j] = k;
                k += 1;
            }
            bottom -= 1;

            for i in (top..bottom).rev() {
                matrix[i][left] = k;
                k += 1;
            }
            left += 1;
        }

        matrix
    }
}


struct MatrixGenerator {
    template: Vec<Vec<Option<i32>>>,
    i: usize,
    j: usize,
    direction: Direction
}

impl MatrixGenerator {
    fn new(n: i32) -> Self {
        let template = vec![vec![None; n as usize]; n as usize];
        Self {
            template,
            i: 0,
            j: 0,
            direction: Direction::RIGHT,
        }
    }

    pub fn spiral(mut self) -> Vec<Vec<i32>> {
        let how_many = self.template.len() as i32 * self.template[0].len() as i32;

        for num in 1..=how_many {
            self.template[self.i][self.j] = Some(num);
            self.step();
            // println!("we're {:?} and at {},{}", self.direction, self.i, self.j);
            // println!("{:?}", self.template);
        }

        self.template
            .iter()
            .map(|v| v.iter().map(|op| op.unwrap()).collect())
            .collect()
    }

    fn step(&mut self) {
        if !self.next_step_allowed() {
            match self.direction {
                Direction::RIGHT => self.direction = Direction::DOWN,
                Direction::DOWN => self.direction = Direction::LEFT,
                Direction::LEFT => self.direction = Direction::UP,
                Direction::UP => self.direction = Direction::RIGHT,
            }
        }

        match self.direction {
            Direction::RIGHT => self.j += 1,
            Direction::DOWN => self.i += 1,
            Direction::LEFT => self.j -= 1,
            Direction::UP => self.i -= 1,
        }
    }

    fn next_step_allowed(&self) -> bool {
        let rows = self.template.len();
        let cols = self.template[0].len();
        let i = self.i;
        let j = self.j;
        
        match self.direction {
            Direction::RIGHT => {
                if j >= cols - 1 || self.template[i][j + 1].is_some() {
                    return false;
                } else {
                    return true;
                }
            },
            Direction::DOWN => {
                if i >= rows - 1 || self.template[i + 1][j].is_some() {
                    return false;
                } else {
                    return true;
                }
            },
            Direction::LEFT => {
                if j <= 0 || self.template[i][j - 1].is_some() {
                    return false;
                } else {
                    return true;
                }
            },
            Direction::UP => {
                if i <= 0 || self.template[i - 1][j].is_some() {
                    return false;
                } else {
                    return true;
                }
            },
        }
    }
}


#[derive(Debug)]
pub enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}


fn main() {
    let m = MatrixGenerator::new(3).spiral();
    println!("{:?}", m);
}