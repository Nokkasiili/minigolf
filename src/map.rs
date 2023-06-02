use crate::tile::{Tile, TileCreationError};

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Tile>,
}

#[derive(Debug)]
pub enum MapError {
    OutOfBounds,
    Unexpected(char),
    UnexpectedEol,
    TileCreationError(TileCreationError),
}
impl From<TileCreationError> for MapError {
    fn from(error: TileCreationError) -> Self {
        MapError::TileCreationError(error)
    }
}
impl Map {
    const HEIGHT: usize = 25;
    const WIDTH: usize = 49;

    pub fn new() -> Self {
        Self {
            tiles: vec![Tile::default(); Map::WIDTH * Map::HEIGHT],
        }
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
                            let a_code = Map::char_to_code(a);
                            let b_code = Map::char_to_code(b);
                            let tile = Tile::from_i32s(Map::char_to_code(cur), a_code, b_code, 0)?;
                            map.set_tile(x, y, tile)?;
                        }
                        'B' => {
                            let a = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let b = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let c = iter.next().ok_or_else(|| MapError::UnexpectedEol)?;
                            let a_code = Map::char_to_code(a);
                            let b_code = Map::char_to_code(b);
                            let c_code = Map::char_to_code(c);
                            let tile =
                                Tile::from_i32s(Map::char_to_code(cur), a_code, b_code, c_code)?;
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

    fn char_to_code(c: char) -> i32 {
        match c {
            'a'..='z' => c as i32 - 'a' as i32 + 26,
            'A'..='Z' => c as i32 - 'A' as i32,
            _ => unreachable!(),
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
