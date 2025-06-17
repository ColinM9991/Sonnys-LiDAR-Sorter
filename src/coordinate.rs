use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum PathError {
    InvalidLength(usize),
    InvalidFileName(String),
}

impl<'a> Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength(len) => f.write_fmt(format_args!(
                "Invalid filename length, expected length of 7 but got {}",
                len
            )),
            Self::InvalidFileName(name) => f.write_fmt(format_args!(
                "Invalid filename format, {} does not match expected format",
                name
            )),
        }
    }
}

pub struct Coordinate {
    lat: i8,
    lon: i8,
}

impl Coordinate {
    pub fn new(lat_coordinate: i8, lon_coordinate: i8) -> Self {
        Self {
            lat: lat_coordinate,
            lon: lon_coordinate,
        }
    }

    pub fn to_grid_position(&self) -> String {
        const DIVISOR: f32 = 10f32;

        let lat = (self.lat as f32 / DIVISOR).floor() * DIVISOR;
        let lon = (self.lon as f32 / DIVISOR).floor() * DIVISOR;

        return format!("{:>+03.}{:>+04.}", lat, lon);
    }

    fn as_direction(letter: char, value: i8) -> i8 {
        match letter {
            'N' | 'E' => value,
            'S' | 'W' => -value,
            _ => panic!("Invalid character specified"),
        }
    }
}

impl FromStr for Coordinate {
    type Err = PathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 7 {
            return Err(PathError::InvalidLength(value.len()));
        }

        let mut lat = value[..3].chars();
        let mut lon = value[3..].chars();

        let lat_direction = lat.next().unwrap();
        let lon_direction = lon.next().unwrap();

        let lat = parse_coordinate(lat, value)?;
        let lon = parse_coordinate(lon, value)?;

        Ok(Self::new(
            Self::as_direction(lat_direction, lat),
            Self::as_direction(lon_direction, lon),
        ))
    }
}

fn parse_coordinate<'a>(value: std::str::Chars, file_name: &'a str) -> Result<i8, PathError> {
    value
        .collect::<String>()
        .parse::<i8>()
        .or(Err(PathError::InvalidFileName(file_name.into())))
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_lat_lon_grid_expected() {
        let lat_long = "N78E030".parse::<Coordinate>();
        assert_eq!("+70+030", lat_long.unwrap().to_grid_position());

        let lat_long = "N63W023".parse::<Coordinate>();
        assert_eq!("+60-030", lat_long.unwrap().to_grid_position());

        let lat_long = "N47E011".parse::<Coordinate>();
        assert_eq!("+40+010", lat_long.unwrap().to_grid_position());
    }

    #[test]
    fn latlong_from_string_expected() {
        assert!(Coordinate::new(43, -20) == "N43W020".parse().unwrap());
    }
}
