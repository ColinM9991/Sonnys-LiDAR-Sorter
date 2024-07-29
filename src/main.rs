use std::error::Error;

use clap::Parser;
use sonny_sorter::Paths;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = Paths::parse();

    sonny_sorter::run(paths)?;

    Ok(())
}
