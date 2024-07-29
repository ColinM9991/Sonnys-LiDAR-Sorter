mod coordinate;
mod paths;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use paths::Paths;

use crate::coordinate::*;
pub use crate::paths::CliPaths;

pub fn run<T: Paths>(config: T) -> Result<(), Box<dyn Error>> {
    let mut file_map: HashMap<String, HashSet<PathBuf>> = HashMap::new();

    for file in config.get_lidar_files()? {
        let file_stem = file.file_stem().unwrap().to_str().unwrap();
        let coordinate = match Coordinate::try_from(file_stem) {
            Ok(res) => res,
            Err(err) => panic!(
                "An error occurred when handling '{}'. Error was: {}",
                file_stem, err
            ),
        };

        let coordinate = coordinate.to_grid_position();

        file_map.entry(coordinate).or_default().insert(file);
    }

    for (mapping, files) in file_map.into_iter() {
        let elevation_path = config.get_lidar_path().join(&mapping);
        let sym_link_path = config.get_elevation_data_path().join(&mapping);

        if !elevation_path.exists() {
            fs::create_dir(&elevation_path)?;
        }

        if !sym_link_path.exists() {
            create_symlink(&elevation_path, &sym_link_path)?;
        }

        for file in files {
            let new_file_path = elevation_path.join(&file.file_name().unwrap());

            fs::rename(&file, &new_file_path).expect(&format!(
                "Couldn't move '{}' to '{}'",
                file.to_string_lossy().to_string(),
                new_file_path.to_string_lossy().to_string()
            ));
        }
    }

    Ok(())
}

// Creates a junction using CMD since Windows considers creating a symlink, via std::os::windows::fs::symlink_dir, to be privileged action
// Not really keen on asking people to run an app via Administrator mode since that's not really a secure solution.
#[cfg(windows)]
fn create_symlink(source: &PathBuf, target: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(std::path::absolute(target).unwrap().to_str().unwrap())
        .arg(std::path::absolute(source).unwrap().to_str().unwrap())
        .output()
        .map(|_| Ok(()))?
}

#[cfg(unix)]
fn create_symlink(source: &PathBuf, target: &PathBuf) -> Result<(), std::io::Error> {
    std::os::unix::fs::symlink(
        std::path::absolute(source).unwrap(),
        std::path::absolute(target).unwrap(),
    )
}
