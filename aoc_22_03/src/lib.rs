// ----------------------------------------------------
// modules
// ----------------------------------------------------
mod model;
mod reader;
mod utils;

// ----------------------------------------------------
// imports and re-exports
// ----------------------------------------------------
use std::path::Path;

// ----------------------------------------------------
pub use model::{Compartment, Item, Knapsack};
pub use reader::Reader;
pub use utils::*;

// ----------------------------------------------------
// starter func
// ----------------------------------------------------
pub fn start_app() {
    let knapsacks = Path::new("./input/aoc_22_03/input.txt").read();

    println!(
        "Sum of priorities of wrongly sorted items is {}.",
        sum_of_wrong_item_priorities(&knapsacks)
    );

    println!(
        "Sum of priorities of elf group common items is {}.",
        sum_common_item_priorities(&knapsacks)
    )
}
