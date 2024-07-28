use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io, path::Path};

use sonny_sorter::Coordinate;

fn main() -> Result<(), Box<dyn Error>> {
    let mut hgt_directory = String::new();
    let mut elevation_data_directory = String::new();

    println!("Enter the full path for where the HGT files are stored:");
    io::stdin()
        .read_line(&mut hgt_directory)
        .expect("Did not specify a HGT location");

    println!("Enter the full path to the Ortho4XP Elevation_data directory:");
    io::stdin()
        .read_line(&mut elevation_data_directory)
        .expect("Did not specify an elevation data directory");

    let hgt_path = Path::new(hgt_directory.trim());
    let elevation_data_path = Path::new(elevation_data_directory.trim());

    if !hgt_path.exists() || !elevation_data_path.exists() {
        panic!()
    };

    let files = match hgt_path.read_dir() {
        Ok(files) => files
            .filter_map(|file| file.ok())
            .map(|entry| entry.path())
            .filter_map(|path| {
                if path.extension().map_or(false, |ext| ext == "hgt") {
                    Some(path)
                } else {
                    None
                }
            }),
        Err(err) => panic!(
            "Couldn't enumerate directory {}. Error {}",
            hgt_directory, err
        ),
    };

    let mut file_map: HashMap<String, HashSet<PathBuf>> = HashMap::new();

    for file in files {
        let file_stem = file.file_stem().unwrap().to_str().unwrap();
        let coordinate = match Coordinate::try_from(file_stem) {
            Ok(res) => res,
            Err(err) => panic!("Error handling '{}'. Error was: {}", file_stem, err),
        };

        let coordinate = coordinate.to_grid_position();

        file_map.entry(coordinate).or_default().insert(file);
    }

    for (mapping, files) in file_map {
        let elevation_path = &hgt_path.join(&mapping);
        let sym_link_path = &elevation_data_path.join(&mapping);
        
        if !elevation_path.exists() {
            fs::create_dir(elevation_path)?;
        }

        if !sym_link_path.exists() {
            create_symlink(elevation_path, sym_link_path)?;
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

    println!("Finished");

    Ok(())
}

// Creates a junction using CMD since Windows considers creating a symlink, via std::os::windows::fs::symlink_dir to be privileged action
// Not really keen on asking people to run an app via Administrator mode since that's not really a secure solution.
#[cfg(windows)]
fn create_symlink(source: &PathBuf, target: &PathBuf) -> Result<(), io::Error> {
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(target.to_str().unwrap())
        .arg(source.to_str().unwrap())
        .output()
        .map(|_| Ok(()))?
}

#[cfg(unix)]
fn create_symlink(source: &PathBuf, target: &PathBuf) -> Result<(), io::Error> {
    std::os::unix::fs::symlink_dir(source, target)
}
