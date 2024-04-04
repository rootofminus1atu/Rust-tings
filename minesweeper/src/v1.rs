use std::collections::HashMap;
use rand::seq::SliceRandom;
use tracing::info;
use ndarray::Array2;

#[derive(Debug, Clone)]
pub enum MinesweeperCell {
    Num(i32),
    Bomb
}

impl MinesweeperCell {
    pub fn increment_if_possible(&mut self) {
        if let Self::Num(n) = self {
           *n += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct MinesweeperGrid {
    pub cells: Array2<MinesweeperCell>
}

impl MinesweeperGrid {
    pub fn even_better(rows: usize, cols: usize, bombs: usize) -> Self {
        let mut special_coords = HashMap::new();

        let bomb_coords = n_random_coords(rows, cols, bombs);

        bomb_coords.iter().for_each(|&c| {
            special_coords.insert(c, MinesweeperCell::Bomb);
        });

        bomb_coords.iter()
            .flat_map(|bomb_coord| get_neighbors(bomb_coord, rows, cols))
            .for_each(|c| {
                special_coords.entry(c)
                    .and_modify(|cell| cell.increment_if_possible())
                    .or_insert(MinesweeperCell::Num(1));
            });

        let cells = Array2::from_shape_fn((rows, cols), |(i, j)| {
            special_coords.get(&(i, j)).unwrap_or(&MinesweeperCell::Num(0)).clone()
        });

        Self { cells }
    }

    pub fn new_better(rows: usize, cols: usize, bombs: usize) -> Self {
        let mut special_coords = HashMap::new();

        let bomb_coords = n_random_coords(rows, cols, bombs);

        let num_coords = bomb_coords.iter()
            .map(|bomb_coord| get_neighbors(bomb_coord, rows, cols))
            .flat_map(|list| list)
            .collect::<Vec<_>>();

        for c in &bomb_coords {
            special_coords.insert(*c, MinesweeperCell::Bomb);
        }
        for c in &num_coords {
            special_coords.entry(*c)
                .and_modify(|cell| cell.increment_if_possible())
                .or_insert(MinesweeperCell::Num(1));
        }

        info!("{:?}", special_coords);

        let cells = Array2::from_shape_fn((rows, cols), |(i, j)| {
            special_coords.get(&(i, j)).unwrap_or(&MinesweeperCell::Num(0)).clone()
        });

        Self { cells }
    }

    pub fn new(rows: usize, cols: usize, bombs: usize) -> Self {
        let bomb_coords = n_random_coords(rows, cols, bombs);

        let mut cells = Array2::from_elem((rows, cols), MinesweeperCell::Num(0));

        for coord in &bomb_coords {
            cells[*coord] = MinesweeperCell::Bomb;

            let neighbor_coords = get_neighbors(coord, rows, cols);
            info!("{:?}", neighbor_coords);
            
            for neighbor in &neighbor_coords {
                if let MinesweeperCell::Num(n) = cells[*neighbor] {
                    cells[*neighbor] = MinesweeperCell::Num(n + 1);
                }
            }
        }

        Self { cells }
    }
}

pub fn get_neighbors(coord: &(usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    // (3, 7)
    // -> (2 to 4, 6 to 8) without (3, 7) 
    let lower = (coord.0.checked_sub(1).unwrap_or(0), coord.1.checked_sub(1).unwrap_or(0));
    let upper = (
        if coord.0 == rows - 1 { coord.0 } else { coord.0 + 1 }, 
        if coord.1 == cols - 1 { coord.1 } else { coord.1 + 1 }
    );

    info!("center: {:?}, bounds: {:?} - {:?}", coord, lower, upper);

    (lower.0..(upper.0 + 1))
    .flat_map(|r| (lower.1..(upper.1 + 1)).map(move |c| (r, c)))
    .filter(|(r, c)| !(*r == coord.0 && *c == coord.1))
    .collect::<Vec<(usize, usize)>>()
}


// todo - make this work
pub fn generate_coords(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    (from.0..to.0)
    .flat_map(|r| (from.1..to.1).map(move |c| (r, c)))
    .collect::<Vec<(usize, usize)>>()
}

pub fn n_random_coords(rows: usize, cols: usize, n: usize) -> Vec<(usize, usize)> {
    let mut all_coords = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect::<Vec<(usize, usize)>>();

    random_shuffle(&mut all_coords);

    // todo - uhhh something so that this doesnt panic
    all_coords[0..n].to_vec()
}

pub fn random_shuffle<T>(list: &mut [T]) {
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
}