use std::{collections::HashSet, io::Write};

use tracing::info;
use tracing_subscriber;
use v1::MinesweeperCell;
use v2::{Coord, DimensionsWithBombs, DimensionsWithBombsAmount, NonZeroDimensions};

mod v1;
mod v2;
mod v3;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let m = v2::Minesweeper::new_empty(v2::NonZeroDimensions::parse(5, 5)?);

    loop {
        m.show();
        println!("enter coords");

        let coords = loop {
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            
            std::io::stdin()
                .read_line(&mut input)
                .expect("bruh your terminal brojen");

            let res = match input.trim().split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>() {
                Ok(mut nums) if nums.len() == 2 => (nums.remove(0), nums.remove(0)),
                _ => continue
            };

            println!("we got {:?}", res);

            break res;
        };

        println!("entered: {:?}", coords);
    }

    Ok(())
}
