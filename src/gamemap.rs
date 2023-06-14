use crate::array2diter::Array2DRangeIterator;
use crate::map::Map;
use crate::tile::{Element, Shape, Special, Tile};
use crate::vector2d::Vector2D;
use image::GenericImageView;
use image::ImageError;
use image::Pixel;
use std::num::TryFromIntError;
use thiserror::Error;

//Used in physics
pub struct GameMap {
    pub tiles: Vec<GameMapTile>,
}
#[derive(Debug)]
pub enum GameMapTile {
    Special(Special),
    Element(Element),
}

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("Image Error:{0}")]
    ImageError(#[from] ImageError),
    #[error("Try From Int Error{0}")]
    TryFromIntError(#[from] TryFromIntError),
}

pub struct Asset {
    sprites: Vec<Vec<bool>>,
}

pub struct Assets {
    specials: Asset,
    shapes: Asset,
}

impl Assets {
    pub const SHAPEPATH: &str = "./assets/shapes.png";
    pub const SPECIALPATH: &str = "./assets/specials.png";

    pub fn new() -> Result<Self, AssetError> {
        let specials = Asset::load(Assets::SPECIALPATH, 28)?;
        let shapes = Asset::load(Assets::SHAPEPATH, 28)?;
        Ok(Self { specials, shapes })
    }
}

impl Asset {
    pub fn load(path: &str, len: usize) -> Result<Self, AssetError> {
        let image = image::open(path)?;
        let mut sprites = Vec::new();

        for i in 0..len {
            let mut sprite = Vec::new();
            for y in 0..Map::TILESIZE {
                for x in 0..Map::TILESIZE {
                    let x_pos: u32 = (i * Map::TILESIZE + x).try_into()?;
                    let y_pos: u32 = y.try_into()?;
                    let pixel = image.get_pixel(x_pos, y_pos).to_rgba();
                    sprite.push(pixel[3] != 0);
                }
            }
            sprites.push(sprite);
        }
        Ok(Self { sprites })
    }

    pub fn get(&self, i: usize) -> Option<Vec<bool>> {
        self.sprites.get(i).cloned()
    }

    pub fn get_bool_xy(&self, i: usize, x: usize, y: usize) -> bool {
        let pix = y * Map::TILESIZE + x;
        self.get(i).unwrap().get(pix).unwrap().to_owned()
    }
}

impl GameMap {
    pub const HEIGHT: usize = Map::HEIGHT * Map::TILESIZE;
    pub const WIDTH: usize = Map::WIDTH * Map::TILESIZE;

    /*
    pub fn new() -> Self {
        Self {
            tiles: vec![; GameMap::WIDTH * FullMap::HEIGHT],
        }
    }*/

    fn maptile_from_tile(tile: &Tile, assets: &Assets, x: usize, y: usize) -> GameMapTile {
        match tile.special {
            None => {
                let i = tile.shape.unwrap() as usize;
                let shape = assets.shapes.get_bool_xy(i, x, y);
                match shape {
                    true => GameMapTile::Element(tile.foreground),
                    false => GameMapTile::Element(tile.background),
                }
            }
            Special => {
                let special = Special.unwrap();
                match special {
                    //These will use background tile in friction calcs and etc
                    Special::FakeHole
                    | Special::RedTeleportExit
                    | Special::BlueTeleportExit
                    | Special::GreenTeleportExit
                    | Special::YellowTeleportExit
                    | Special::MagnetRepel => GameMapTile::Element(tile.background),

                    //Breakable blocks have big hitbox
                    Special::HalfBreakable
                    | Special::QuaterBreakable
                    | Special::FullBreakable
                    | Special::ThreeQuaterBreakable => GameMapTile::Special(special),

                    Special::MagnetAttract => match tile.background {
                        Element::Acid
                        | Element::Water
                        | Element::AcidSwamp
                        | Element::WaterSwamp => GameMapTile::Element(tile.background),
                        _ => GameMapTile::Special(special),
                    },

                    //In all other cases use shape
                    _ => {
                        let i = tile.special.unwrap() as usize;
                        match assets.specials.get_bool_xy(i, x, y) {
                            true => GameMapTile::Special(special),
                            false => GameMapTile::Element(tile.background),
                        }
                    }
                }
            }
        }
    }

    pub fn from_map(map: &Map, assets: &Assets) -> Self {
        let mut tiles = Vec::new();
        for (_, x, y) in
            Array2DRangeIterator::<usize>::new(0..GameMap::WIDTH * GameMap::HEIGHT, GameMap::WIDTH)
        {
            if let Some(tile) = map.get_tile(x / Map::TILESIZE, y / Map::TILESIZE) {
                tiles.push(GameMap::maptile_from_tile(
                    &tile,
                    assets,
                    x % Map::TILESIZE,
                    y % Map::TILESIZE,
                ));
            }
        }
        Self { tiles }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&GameMapTile> {
        if x < GameMap::WIDTH && y < GameMap::HEIGHT {
            Some(&self.tiles[y * GameMap::WIDTH + x])
        } else {
            None
        }
    }
}
