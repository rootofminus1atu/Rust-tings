use std::collections::HashSet;
use rand::seq::SliceRandom;
use tracing::{debug, info};


#[derive(thiserror::Error, Debug, Clone)]
pub enum MinesweeperError {
    #[error("Too many bombs")]
    TooManyBombs,
    #[error("Bombs are outside the bounds of the grid")]
    BombsOutsideBounds,
    #[error("Grid dimensions cannot be 0")]
    InvalidDimensions,
}


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



#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Coord {
    i: usize,
    j: usize
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    /// Returns an iterator over the neighbors of the coordinate.
    ///
    /// The neighbors of a coordinate are the 8 cells that surround it, if they exist. This method takes into account the dimensions of the grid to ensure that it doesn't return coordinates outside the grid.
    ///
    /// # Arguments
    ///
    /// * `rows`: The number of rows in the grid.
    /// * `cols`: The number of columns in the grid.
    ///
    /// # Returns
    ///
    /// An iterator over the neighbors of the coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// let coord = Coord::new(1, 2);
    /// let neighbors = coord.neighbors(3, 3);
    /// for neighbor in neighbors {
    ///     println!("{:?}", neighbor);
    /// }
    /// ```
    ///
    /// This will print:
    ///
    /// ```
    /// Coord { i: 0, j: 1 }
    /// Coord { i: 0, j: 2 }
    /// Coord { i: 0, j: 3 }
    /// Coord { i: 1, j: 1 }
    /// Coord { i: 1, j: 3 }
    /// Coord { i: 2, j: 1 }
    /// Coord { i: 2, j: 2 }
    /// Coord { i: 2, j: 3 }
    /// ```
    ///
    /// # Visual Representation
    ///
    /// Here's a simple representation of a cell and its neighbors in a grid. The filled square (■) represents the cell in question, and the empty squares (□) represent its neighbors.
    ///
    /// ```
    /// □ □ □
    /// □ ■ □
    /// □ □ □
    /// ```
    ///
    /// In this representation, the cell at the center (■) has eight neighbors (□), assuming it's not on the edge of the grid. If the cell is on the edge or in a corner of the grid, it will have fewer neighbors. For example, a cell in the corner of the grid would have only three neighbors:
    ///
    /// ```
    /// ■ □
    /// □ □
    /// ```
    ///
    /// And a cell on the edge of the grid (but not in a corner) would have five neighbors:
    ///
    /// ```
    /// □ ■ □
    /// □ □ □
    /// ```
    pub fn neighbors(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        let lower_i = self.i.saturating_sub(1);
        let lower_j = self.j.saturating_sub(1);
        let upper_i = (rows - 1).min(self.i + 1);
        let upper_j = (cols - 1).min(self.j + 1);

        debug!("center: {:?}, bounds: {:?} - {:?}", self, (lower_i, lower_j), (upper_i, upper_j));

        // let idk = (lower_i..upper_i)
        // .flat_map(move |r| {
        //     (lower_j..upper_j).filter_map(move |c| {
        //         if r == self.i && c == self.j {
        //             None
        //         } else {
        //             Some(Self::new(r, c))
        //         }
        //     })
        // });

        (lower_i..=upper_i)
        .flat_map(move |r| (lower_j..=upper_j).map(move |c| Self::new(r, c)))
        .filter(move |coord| coord != self)
    }
}

#[derive(Debug, Clone)]
pub struct DimensionsWithBombsAmount {
    rows: usize,
    cols: usize,
    amount: usize
}

impl DimensionsWithBombsAmount {
    pub fn parse(rows: usize, cols: usize, amount: usize) -> Result<Self, MinesweeperError> {
        if rows == 0 || cols == 0 {
            return Err(MinesweeperError::InvalidDimensions);
        }

        if amount > rows * cols {
            return Err(MinesweeperError::TooManyBombs);
        }

        Ok(Self { rows, cols, amount })
    }

    pub fn n_random_coords(&self) -> HashSet<Coord> {
        let mut all_coords = (0..self.rows)
            .flat_map(|i| (0..self.cols).map(move |j| Coord::new(i, j)))
            .collect::<Vec<_>>();
    
        random_shuffle(&mut all_coords);
    
        // won't ever panic beacuse first it has to be verified by ::parse()
        all_coords[0..self.amount].iter().cloned().collect()
    }
    
}

pub fn random_shuffle<T>(list: &mut [T]) {
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
}

#[derive(Debug, Clone)]
pub struct DimensionsWithBombs {
    rows: usize,
    cols: usize,
    bombs: HashSet<Coord>
}

impl DimensionsWithBombs {
    pub fn parse(rows: usize, cols: usize, bombs: HashSet<Coord>) -> Result<Self, MinesweeperError> {
        DimensionsWithBombsAmount::parse(rows, cols, bombs.len())?;

        if bombs.iter().any(|bomb_coord| bomb_coord.i >= rows || bomb_coord.j >= cols) {
            return Err(MinesweeperError::BombsOutsideBounds);
        }

        Ok(Self { rows, cols, bombs })
    }

    pub fn new_with_random_bombs(dims_with_amounts: DimensionsWithBombsAmount) -> Self {
        let bombs = dims_with_amounts.n_random_coords();

        let DimensionsWithBombsAmount {
            rows,
            cols,
            ..
        } = dims_with_amounts;
        
        Self { rows, cols, bombs }
    }
}


#[derive(Debug, Clone)]
pub struct Minesweeper {
    pub cells: Vec<Vec<MinesweeperCell>>
}

impl Minesweeper {
    pub fn from_dims_with_bombs(dims_with_bombs: DimensionsWithBombs) -> Self {
        let rows = dims_with_bombs.rows;
        let cols = dims_with_bombs.cols;
        let bombs = dims_with_bombs.bombs;

        let mut cells = vec![vec![MinesweeperCell::Num(0); cols]; rows];

        bombs.iter()
            .for_each(|b| cells[b.i][b.j] = MinesweeperCell::Bomb);

        debug!("bomb cells: {:?}", cells);

        bombs.iter()
            .flat_map(|b| b.neighbors(rows, cols))
            .for_each(|b| {
                let cell = &mut cells[b.i][b.j];
                cell.increment_if_possible();
            });

        debug!("bomb and num cells: {:?}", cells);

        Self { cells }
    }

    pub fn new_random(rows: usize, cols: usize, amount: usize) -> Result<Self, MinesweeperError> {
        let dims_with_amounts = DimensionsWithBombsAmount::parse(rows, cols, amount)?;

        let dims_with_bombs = DimensionsWithBombs::new_with_random_bombs(dims_with_amounts);

        Ok(Self::from_dims_with_bombs(dims_with_bombs))
    }

    pub fn new_empty(rows: usize, cols: usize) -> Result<Self, MinesweeperError> {
        // could prob extract this check into something like NonZeroDimensions::parse()
        if rows == 0 || cols == 0 {
            return Err(MinesweeperError::InvalidDimensions);
        }

        let cells = vec![vec![MinesweeperCell::Num(0); cols]; rows];

        Ok(Self { cells })
    }

    // should be overrideable
    pub fn show(&self) {
        for row in &self.cells {
            let row_string: Vec<String> = row.iter().map(|cell| {
                match cell {
                    MinesweeperCell::Num(n) => format!("{} ", n),
                    MinesweeperCell::Bomb => format!("B "),
                }
            }).collect();
            println!("{}", row_string.join(" "));
        }
    }
}
