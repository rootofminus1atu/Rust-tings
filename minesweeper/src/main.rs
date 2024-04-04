use std::collections::HashSet;

use tracing::info;
use tracing_subscriber;
use v2::DimensionsWithBombsAmount;

mod v1;
mod v2;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // let m = v1::MinesweeperGrid::new_better(3, 3, 2);
    // info!("we got\n{:?}", m.cells);

    // let h = v2::DimensionsWithBombs::parse(3, 5, 
    //     HashSet::from([v2::Coord::new(0, 0), v2::Coord::new(0, 2)])
    // )?;
    // let m = v2::Minesweeper::from_dims_with_bombs(h);


    let a = v2::DimensionsWithBombs::new_with_random_bombs(
        DimensionsWithBombsAmount::parse(5, 5, 6)?
    );
    let b = v2::Minesweeper::from_dims_with_bombs(a);
    b.show();

    Ok(())
}
