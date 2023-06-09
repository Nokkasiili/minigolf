use crate::tile::{Tile, TileCreationError};
use crate::vector2d::Vector2D;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub ads: Vec<Ad>,
}

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum AdSize {
    Small,
    Medium,
    Large,
    Full,
}

#[derive(Debug)]
pub struct Ad {
    size: AdSize,
    x: i32,
    y: i32,
}

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Out of Bounds")]
    OutOfBounds,
    #[error("Unexpected char {0}")]
    Unexpected(char),
    #[error("Unexpected end of line")]
    UnexpectedEol,
    #[error("TileCreation Error")]
    TileCreationError(#[from] TileCreationError),
    #[error("ParseInt Error")]
    ParseIntError(#[from] ParseIntError),
}

impl AdSize {
    pub fn get_ad_size(size: &Self) -> Vector2D<i32> {
        match size {
            AdSize::Small => Vector2D { x: 3, y: 2 },
            AdSize::Medium => Vector2D { x: 5, y: 3 },
            AdSize::Large => Vector2D { x: 8, y: 5 },
            AdSize::Full => Vector2D { x: 49, y: 25 },
        }
    }
}

impl Ad {
    pub fn from_string(input: &str) -> Result<Vec<Ad>, MapError> {
        let mut ads = Vec::new();
        for chunk in input.chars().collect::<Vec<char>>().chunks(5) {
            if let Some((first_char, last_chars)) = chunk.split_first() {
                let ad_code = Map::char_to_code(*first_char)
                    .ok_or_else(|| MapError::Unexpected(*first_char))?;
                if last_chars.len() != 4 {
                    return Err(MapError::UnexpectedEol);
                }
                let size: AdSize = FromPrimitive::from_i32(ad_code)
                    .ok_or_else(|| MapError::Unexpected(*first_char))?;
                let x = last_chars[..2].iter().collect::<String>().parse::<i32>()?;
                let y = last_chars[2..].iter().collect::<String>().parse::<i32>()?;
                ads.push(Ad { size, x, y })
            }
        }
        Ok(ads)
    }
}

impl Map {
    pub const HEIGHT: usize = 25;
    pub const TILESIZE: usize = 15;
    pub const WIDTH: usize = 49;

    pub fn new() -> Self {
        Self {
            tiles: vec![Tile::default(); Map::WIDTH * Map::HEIGHT],
            ads: Vec::new(),
        }
    }

    pub fn from_string(input: &str) -> Result<Map, MapError> {
        let mut split = input.split(",Ads:");
        let map_str = split.next().unwrap_or("");
        let ads_str = split.next().unwrap_or("");
        let decompressed = Map::decompress(map_str);
        let mut map = Map::decode(decompressed)?;
        map.ads = Ad::from_string(ads_str)?;
        Ok(map)
    }

    pub fn decompress(input: &str) -> String {
        let mut output = String::new();
        let mut count = String::new();

        for c in input.chars() {
            if c.is_digit(10) {
                count.push(c);
            } else {
                let repeat_count = count.parse::<usize>().unwrap_or(1);
                output.push_str(&c.to_string().repeat(repeat_count));
                count.clear();
            }
        }

        output
    }

    pub fn compress(input: &str) -> String {
        let mut compressed_string = String::new();
        let mut count = 1;
        let chars: Vec<char> = input.chars().collect();

        for i in 0..chars.len() {
            if i + 1 < chars.len() && chars[i] == chars[i + 1] {
                count += 1;
            } else {
                if count > 1 {
                    compressed_string.push_str(&count.to_string());
                }
                compressed_string.push(chars[i]);
                count = 1;
            }
        }

        compressed_string
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) -> Result<(), MapError> {
        if x < Map::WIDTH && y < Map::HEIGHT {
            self.tiles[y * Map::WIDTH + x] = tile;
            Ok(())
        } else {
            Err(MapError::OutOfBounds)
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < Map::WIDTH && y < Map::HEIGHT {
            Some(self.tiles[y * Map::WIDTH + x])
        } else {
            None
        }
    }

    pub fn index_to_xy(index: usize) -> (usize, usize) {
        let y = index / (Map::WIDTH);
        let x = index % (Map::WIDTH);
        (x, y)
    }

    pub fn decode(s: String) -> Result<Map, MapError> {
        let mut map = Map::new();
        let mut iter = s.chars();

        for y in 0..Map::HEIGHT {
            for x in 0..Map::WIDTH {
                if let Some(cur) = iter.next() {
                    match cur {
                        'A' | 'C' => {
                            let a = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let b = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let a_code =
                                Map::char_to_code(a).ok_or_else(|| MapError::Unexpected(a))?;
                            let b_code =
                                Map::char_to_code(b).ok_or_else(|| MapError::Unexpected(b))?;
                            let cur =
                                Map::char_to_code(cur).ok_or_else(|| MapError::Unexpected(b))?;
                            let tile = Tile::from_i32s(cur, a_code, b_code, 0)?;
                            map.set_tile(x, y, tile)?;
                        }
                        'B' => {
                            let a = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let b = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let c = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let a_code =
                                Map::char_to_code(a).ok_or_else(|| MapError::Unexpected(a))?;
                            let b_code =
                                Map::char_to_code(b).ok_or_else(|| MapError::Unexpected(b))?;
                            let c_code =
                                Map::char_to_code(c).ok_or_else(|| MapError::Unexpected(c))?;
                            let cur =
                                Map::char_to_code(cur).ok_or_else(|| MapError::Unexpected(b))?;
                            let tile = Tile::from_i32s(cur, a_code, b_code, c_code)?;
                            map.set_tile(x, y, tile)?;
                        }
                        'D' | 'E' | 'F' | 'G' | 'H' | 'I' => {
                            let (offset_y, offset_x) = Map::get_offset(cur);
                            let new_y = y
                                .checked_sub(offset_y)
                                .ok_or_else(|| MapError::OutOfBounds)?;
                            let new_x = x
                                .checked_sub(offset_x)
                                .ok_or_else(|| MapError::OutOfBounds)?;
                            map.set_tile(
                                x,
                                y,
                                map.get_tile(new_x, new_y)
                                    .ok_or_else(|| MapError::OutOfBounds)?,
                            )?;
                        }
                        c => return Err(MapError::Unexpected(c)),
                    }
                }
            }
        }

        Ok(map)
    }

    fn char_to_code(c: char) -> Option<i32> {
        match c {
            'a'..='z' => Some(c as i32 - 'a' as i32 + 26),
            'A'..='Z' => Some(c as i32 - 'A' as i32),
            _ => None,
        }
    }

    fn get_offset(cur: char) -> (usize, usize) {
        match cur {
            'D' => (0, 1),
            'E' => (1, 0),
            'F' => (1, 1),
            'G' => (0, 2),
            'H' => (2, 0),
            'I' => (2, 2),
            _ => (0, 0),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_code() {
        assert_eq!(Map::char_to_code('a'), Some(26));
        assert_eq!(Map::char_to_code('z'), Some(51));
        assert_eq!(Map::char_to_code('A'), Some(0));
        assert_eq!(Map::char_to_code('Z'), Some(25));
        assert_eq!(Map::char_to_code('!'), None);
    }

    #[test]
    fn test_from_string() {
        let input = "A2309B2208C4019";
        let expected_output = vec![
            Ad {
                size: AdSize::Small,
                x: 23,
                y: 9,
            },
            Ad {
                size: AdSize::Medium,
                x: 22,
                y: 8,
            },
            Ad {
                size: AdSize::Large,
                x: 40,
                y: 19,
            },
        ];

        let result = Ad::from_string(input).unwrap();
        assert_eq!(result.len(), expected_output.len());

        for (actual, expected) in result.iter().zip(expected_output.iter()) {
            assert_eq!(actual.size, expected.size);
            assert_eq!(actual.x, expected.x);
            assert_eq!(actual.y, expected.y);
        }
    }
}
