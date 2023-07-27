use crate::sled_basic::{basic, merge_operator};
use crate::sled_structured::upsert;

use sled::{open, Result};

mod sled_basic;
mod sled_structured;

fn main() -> Result<()> {
    println!("Try sled!");

    let _ = basic();
    let _ = merge_operator();

    let db = open("db")?;
    upsert(&db)?;

    Ok(())
}
