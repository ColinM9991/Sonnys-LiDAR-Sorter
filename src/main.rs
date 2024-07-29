use std::error::Error;

use clap::Parser;
use sonny_sorter::CliPaths;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = CliPaths::parse();

    sonny_sorter::run(paths)?;

    Ok(())
}
