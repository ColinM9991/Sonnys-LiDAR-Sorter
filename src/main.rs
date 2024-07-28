use std::error::Error;

use sonny_sorter::ElevationPaths;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = ElevationPaths::build(&mut std::env::args().skip(1).take(2));

    sonny_sorter::run(paths)?;

    println!("Finished");

    Ok(())
}