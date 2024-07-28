use std::fmt::Display;

#[derive(Debug)]
pub enum PathError<'a> {
    InvalidLength(usize),
    InvalidFileName(&'a str),
}

impl<'a> Display for PathError<'a> {
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
    pub fn new(
        (lat_direction, lat_coordinate): (char, i8),
        (lon_direction, lon_coordinate): (char, i8),
    ) -> Self {
        Self {
            lat: Self::as_direction(lat_direction, lat_coordinate),
            lon: Self::as_direction(lon_direction, lon_coordinate),
        }
    }

    fn as_direction(letter: char, value: i8) -> i8 {
        match letter {
            'N' | 'E' => value,
            'S' | 'W' => -value,
            _ => panic!("Invalid character specified"),
        }
    }

    pub fn to_grid_position(&self) -> String {
        const DIVISOR: f32 = 10f32;

        let lat = (self.lat as f32 / DIVISOR).floor() * DIVISOR;
        let lon = (self.lon as f32 / DIVISOR).floor() * DIVISOR;

        return format!("{:>+03.}{:>+04.}", lat, lon);
    }
}

impl<'a> TryFrom<&'a str> for Coordinate {
    type Error = PathError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() != 7 {
            return Err(PathError::InvalidLength(value.len()));
        }

        let mut lat = value[..3].chars();
        let mut lon = value[3..].chars();

        let lat_direction = lat.next().unwrap();
        let lon_direction = lon.next().unwrap();

        let lat = match lat.collect::<String>().parse::<i8>() {
            Ok(val) => val,
            Err(_) => return Err(PathError::InvalidFileName(&value)),
        };

        let lon = match lon.collect::<String>().parse::<i8>() {
            Ok(val) => val,
            Err(_) => return Err(PathError::InvalidFileName(&value)),
        };

        Ok(Self::new((lat_direction, lat), (lon_direction, lon)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_lat_lon_grid_expected() {
        let lat_long = Coordinate::try_from("N78E030");
        assert_eq!("+70+030", lat_long.unwrap().to_grid_position());
    }

    #[test]
    fn latlong_from_string_expected() {
        assert_eq!(-20, Coordinate::try_from("N43W020").unwrap().lon);
    }
}
