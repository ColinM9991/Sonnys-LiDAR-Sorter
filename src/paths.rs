use std::path::{Path, PathBuf};

use clap::Parser;
use clio::ClioPath;

pub trait Paths {
    fn get_lidar_files(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let source = self.get_lidar_path();
        let files = match source.read_dir() {
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
                source.to_string_lossy(),
                err
            ),
        };

        Ok(files.collect())
    }

    fn get_lidar_path(&self) -> &Path;

    fn get_elevation_data_path(&self) -> &Path;
}

#[derive(Parser)]
pub struct CliPaths {
    /// The path containing Sonny's LiDAR files
    #[arg(short, long)]
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    source: ClioPath,

    /// The Elevation_data path for Ortho4XP
    #[arg(short, long)]
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    destination: ClioPath,
}

impl Paths for CliPaths {
    fn get_lidar_path(&self) -> &Path {
        self.source.path()
    }

    fn get_elevation_data_path(&self) -> &Path {
        self.destination.path()
    }
}
