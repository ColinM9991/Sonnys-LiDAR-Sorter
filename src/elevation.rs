use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Paths {
    /// The path containing Sonny's LiDAR files
    #[arg(short, long)]
    source: PathBuf,

    /// The Elevation_data path for Ortho4XP
    #[arg(short, long)]
    destination: PathBuf,
}

impl Paths {
    pub fn get_lidar_files(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let files = match self.source.read_dir() {
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
                self.source.to_string_lossy(),
                err
            ),
        };

        Ok(files.collect())
    }

    pub fn get_lidar_path(&self) -> &PathBuf {
        &self.source
    }

    pub fn get_elevation_data_path(&self) -> &PathBuf {
        &self.destination
    }
}
