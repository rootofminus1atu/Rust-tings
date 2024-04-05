use std::collections::HashSet;

use tracing::info;
use tracing_subscriber;
use v1::MinesweeperCell;
use v2::{Coord, DimensionsWithBombs, DimensionsWithBombsAmount, NonZeroDimensions};

mod v1;
mod v2;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dims = NonZeroDimensions::parse(2, 5)?;

    let example_empty = v2::Minesweeper::new_empty(dims);
    example_empty.show();
    // 0  0  0  0  0
    // 0  0  0  0  0

    let example_random = v2::Minesweeper::new_random(DimensionsWithBombsAmount::parse(dims, 4)?);
    example_random.show();
    // possible outcome: 
    // 0  1  B  B  3 
    // 0  1  3  B  B

    let bombs = HashSet::from([Coord::new(1, 1), Coord::new(0, 3)]);
    let example_rigged = v2::Minesweeper::new_with_bombs(DimensionsWithBombs::parse(dims, bombs)?);
    example_rigged.show();
    // 1  1  2  B  1
    // 1  B  2  1  1

    Ok(())
}
