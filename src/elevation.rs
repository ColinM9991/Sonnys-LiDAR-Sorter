use std::path::PathBuf;

pub struct ElevationPaths {
    lidar_path: PathBuf,
    elevation_data_path: PathBuf,
}

impl ElevationPaths {
    pub fn build(args: &mut (impl Iterator<Item = String> + ExactSizeIterator)) -> Self {
        if args.len() < 2 {
            panic!("Expected arguments not found");
        }

        let lidar_path = String::from(args.next().unwrap());
        let elevation_data_path = String::from(args.next().unwrap());

        let lidar_path = PathBuf::from(lidar_path.trim());
        let elevation_data_path = PathBuf::from(elevation_data_path.trim());

        if !lidar_path.exists() || !elevation_data_path.exists() {
            panic!("One or more of the specified paths were not found");
        };

        Self {
            lidar_path,
            elevation_data_path,
        }
    }

    pub fn get_lidar_files(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let files = match self.lidar_path.read_dir() {
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
                self.lidar_path.to_string_lossy(),
                err
            ),
        };

        Ok(files.collect())
    }

    pub fn get_lidar_path(&self) -> &PathBuf {
        &self.lidar_path
    }

    pub fn get_elevation_data_path(&self) -> &PathBuf {
        &self.elevation_data_path
    }
}
