use bitflags::bitflags;
use chrono::NaiveDateTime;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use thiserror::Error;

use crate::map::Map;
use crate::map::MapError;

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug)]
pub struct Track {
    pub version: i32,
    pub author: String,
    pub name: String,
    pub categories: TrackTypeFlags,
    pub settings: Settings,
    pub ratings: Vec<i32>,
    pub stroke_info: Vec<i32>,
    pub map: Map,
    pub record: Record,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Map error: {0}")]
    MapError(#[from] MapError),
}

bitflags! {
    #[derive(Debug)]
    pub struct TrackTypeFlags: u32 {
        const BASIC = 0b00000001;
        const HOLEINONE = 0b00001000;
        const LONG = 0b00100000;
        const MODERN = 0b00000100;
        const SHORT = 0b00010000;
        const TRADITIONAL = 0b00000010;
    }
}

impl FromPrimitive for TrackTypeFlags {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            1 => Some(TrackTypeFlags::BASIC),
            2 => Some(TrackTypeFlags::TRADITIONAL),
            3 => Some(TrackTypeFlags::MODERN),
            4 => Some(TrackTypeFlags::HOLEINONE),
            5 => Some(TrackTypeFlags::SHORT),
            6 => Some(TrackTypeFlags::LONG),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }
}

#[derive(Debug)]
pub struct Settings {
    pub magnets_visible: bool,
    pub mines_visible: bool,
    pub teleport_colors: bool,
    pub illusion_wall_shadows: bool,
    pub max_players: i32,
    pub min_players: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            magnets_visible: true,
            mines_visible: false,
            teleport_colors: false,
            illusion_wall_shadows: false,
            max_players: 1,
            min_players: 4,
        }
    }
}

impl FromStr for Settings {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut settings = Settings::default();

        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 6 {
            return Err(ParseError::InvalidFormat);
        }

        settings.mines_visible = chars[0] == 't';
        settings.magnets_visible = chars[1] == 't';
        settings.teleport_colors = chars[2] == 't';
        settings.illusion_wall_shadows = chars[3] == 't';

        let min_players_str: String = chars[4].to_string();
        let max_players_str: String = chars[5].to_string();

        settings.min_players = min_players_str
            .parse()
            .map_err(|_| ParseError::InvalidFormat)?;
        settings.max_players = max_players_str
            .parse()
            .map_err(|_| ParseError::InvalidFormat)?;

        Ok(settings)
    }
}

impl Track {
    fn from_reader<R: BufRead>(reader: &mut R) -> Result<Track, ParseError> {
        let lines = reader.lines();

        let mut track = Track {
            version: 0,
            author: String::new(),
            name: String::new(),
            categories: TrackTypeFlags::empty(),
            settings: Settings::default(),
            ratings: Vec::new(),
            stroke_info: Vec::new(),
            map: Map::default(),
            record: Record {
                name: String::new(),
                timestamp: NaiveDateTime::default(),
            },
        };

        for line in lines {
            let line = line?;
            if line.is_empty() {
                continue; // Skip empty lines
            }

            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() != 2 {
                return Err(ParseError::InvalidFormat);
            }

            let section = parts[0];
            let data = parts[1];

            match section {
                "V" => track.version = data.parse().map_err(|_| ParseError::InvalidFormat)?,
                "A" => track.author = data.to_owned(),
                "N" => track.name = data.to_owned(),
                "C" => {
                    let categories: Vec<i32> = data
                        .split(',')
                        .map(|cat| cat.parse().unwrap_or(0))
                        .collect();

                    let mut categories_flags = TrackTypeFlags::empty();
                    for category in categories {
                        if let Some(category_enum) = TrackTypeFlags::from_i32(category) {
                            categories_flags |= category_enum;
                        } else {
                            return Err(ParseError::InvalidFormat);
                        }
                    }

                    track.categories = categories_flags;
                }
                "S" => {
                    track.settings = data
                        .parse::<Settings>()
                        .map_err(|_| ParseError::InvalidFormat)?;
                }
                "T" => track.map = Map::from_string(data)?,
                "R" => {
                    let ratings: Vec<i32> = data
                        .split(',')
                        .map(|rating| rating.parse().unwrap_or(0))
                        .collect();
                    track.ratings = ratings;
                }
                "B" => {
                    let parts: Vec<&str> = data.splitn(2, ',').collect();
                    if parts.len() != 2 {
                        return Err(ParseError::InvalidFormat);
                    }

                    let name = parts[0];
                    let timestamp = parts[1].parse().map_err(|_| ParseError::InvalidFormat)?;
                    let naive_timestamp = NaiveDateTime::from_timestamp_opt(timestamp, 0);
                    if let Some(naive_timestamp) = naive_timestamp {
                        track.record = Record {
                            name: name.to_owned(),
                            timestamp: naive_timestamp,
                        };
                    } else {
                        return Err(ParseError::InvalidFormat);
                    }
                }
                "I" => {
                    let ratings: Vec<i32> = data
                        .split(',')
                        .map(|rating| rating.parse().unwrap_or(0))
                        .collect();
                    track.ratings = ratings;
                }
                _ => return Err(ParseError::InvalidFormat),
            }
        }

        Ok(track)
    }

    pub fn from_filepath(filepath: &str) -> Result<Track, ParseError> {
        let file = File::open(filepath)?;
        let mut reader = BufReader::new(file);
        Track::from_reader(&mut reader)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_filepath() {
        let filepath = "testi.track";

        let result = Track::from_filepath(filepath);

        assert!(result.is_ok());

        let track = result.unwrap();

        assert_eq!(track.version, 2);
        assert_eq!(track.map.ads.len(), 3);
        // assert_eq!(track.title, "Some Title");
    }
}
