use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use thiserror::Error;

use crate::vector2d::Vector2D;
const MAGIC: f32 = std::f32::consts::FRAC_1_SQRT_2;
const DOWNHILLSPEED: f32 = 0.025;

#[derive(Debug, Error)]
pub enum TileCreationError {
    #[error("Invalid special value: {0}")]
    InvalidSpecial(i32),
    #[error("Invalid shape value: {0}")]
    InvalidShape(i32),
    #[error("Invalid background value: {0}")]
    InvalidBackground(i32),
    #[error("Invalid foreground value: {0}")]
    InvalidForeground(i32),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, FromPrimitive)]
pub enum SpecialParse {
    Normal = 1,
    Special = 2,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, FromPrimitive, Hash)]
pub enum Special {
    StartPosition,        //24 0
    Hole,                 //25 1
    FakeHole,             //26 2
    MoveableBlock,        //27 3
    Mine,                 //28 4
    BlownMine,            //29 5
    BigMine,              //30 6
    BlownBigMine,         //31 7
    BlueTeleportStart,    //32 8
    BlueTeleportExit,     //33 9
    RedTeleportStart,     //34 10
    RedTeleportExit,      //35 11
    YellowTeleportStart,  //36 12
    YellowTeleportExit,   //37 13
    GreenTeleportStart,   //38 14
    GreenTeleportExit,    //39 15
    FullBreakable,        //40 16
    ThreeQuaterBreakable, //41 17
    HalfBreakable,        //42 18
    QuaterBreakable,      //43 19
    MagnetAttract,        //44 20
    MagnetRepel,          //45 21
    MoveableBlock2,       //46 22
    SunkMoveableBlock,    //47 23
    StartPositionBlue,    //48 24
    StartPositionRed,     //49 25
    StartPositionYellow,  //50 26
    StartPositionGreen,   //51 27
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, FromPrimitive, Hash)]
pub enum Element {
    Grass,       //0
    Dirt,        //1
    Mud,         //2
    Ice,         //3
    SpeedN,      //4
    SpeedNE,     //5
    SpeedE,      //6
    SpeedSE,     //7
    SpeedS,      //8
    SpeedSW,     //9
    SpeedW,      //10
    SpeedNW,     //11
    Water,       //12
    Acid,        //13
    WaterSwamp,  //14
    AcidSwamp,   //15
    Block,       //16
    StickyBlock, //17
    BouncyBlock, //18
    FakeBlock,   //19
    OnewayN,     //20
    OnewayE,     //21
    OnewayS,     //22
    OnewayW,     //23
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, FromPrimitive, Hash)]
pub enum Shape {
    Blank,
    BigCircle,
    SmallCircle,
    Diamond,
    TriangleSE,
    TriangleSW,
    TriangleNW,
    TriangleNE,
    RoundedSE,
    RoundedSW,
    RoundedNW,
    RoundedNE,

    RoundedS,
    RoundedE,
    RoundedN,
    RoundedW,

    TriangleN,
    TriangleE,
    TriangleS,
    TriangleW,

    TriangleNS,
    TriangleWE,
    HalfW,
    HalfS,
    QuaterNE,
    QuaterSE,
    QuaterSW,
    QuaterNW,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Tile {
    pub special: Option<Special>,
    pub shape: Option<Shape>,
    pub background: Element,
    pub foreground: Element,
}

impl Special {
    pub fn is_solid(&self) -> bool {
        matches!(
            self,
            Special::MoveableBlock
                | Special::MoveableBlock2
                | Special::HalfBreakable
                | Special::QuaterBreakable
                | Special::FullBreakable
                | Special::ThreeQuaterBreakable
        )
    }

    pub fn get_matching_teleport(&self) -> Option<Special> {
        match self {
            Special::RedTeleportStart => Some(Special::RedTeleportExit),
            Special::BlueTeleportStart => Some(Special::BlueTeleportExit),
            Special::GreenTeleportStart => Some(Special::GreenTeleportExit),
            Special::YellowTeleportStart => Some(Special::YellowTeleportExit),
            _ => None,
        }
    }

    pub fn is_teleport_start(&self) -> bool {
        matches!(
            self,
            Special::YellowTeleportStart
                | Special::RedTeleportStart
                | Special::GreenTeleportStart
                | Special::BlueTeleportStart
        )
    }

    pub fn get_friction(&self) -> f32 {
        match self {
            Special::Hole => 0.96,
            Special::BlownMine => 0.9,
            Special::BlownBigMine => 0.9,
            Special::BlueTeleportStart => 0.9975,
            Special::RedTeleportStart => 0.9975,
            Special::YellowTeleportStart => 0.9975,
            Special::GreenTeleportStart => 0.9975,
            Special::MagnetAttract => 0.9,
            Special::SunkMoveableBlock => 0.9935,
            _ => 1.0,
        }
    }
}

impl Element {
    pub fn is_solid(&self) -> bool {
        matches!(
            self,
            Element::Block
                | Element::StickyBlock
                | Element::BouncyBlock
                | Element::OnewayN
                | Element::OnewayE
                | Element::OnewayS
                | Element::OnewayW
        )
    }

    pub fn is_oneway(&self) -> bool {
        matches!(
            self,
            Element::OnewayN | Element::OnewayE | Element::OnewayS | Element::OnewayW
        )
    }

    pub fn is_downhill(&self) -> bool {
        matches!(
            self,
            Element::SpeedN
                | Element::SpeedNE
                | Element::SpeedE
                | Element::SpeedSE
                | Element::SpeedS
                | Element::SpeedSW
                | Element::SpeedW
                | Element::SpeedNW
        )
    }

    pub fn get_downhill_speed(&self) -> Vector2D<f32> {
        let (y, x) = match self {
            Element::SpeedN => (-DOWNHILLSPEED, 0.0),
            Element::SpeedNE => (-DOWNHILLSPEED * MAGIC, DOWNHILLSPEED * MAGIC),
            Element::SpeedE => (0.0, DOWNHILLSPEED),
            Element::SpeedSE => (DOWNHILLSPEED * MAGIC, DOWNHILLSPEED * MAGIC),
            Element::SpeedS => (DOWNHILLSPEED, 0.0),
            Element::SpeedSW => (DOWNHILLSPEED * MAGIC, -DOWNHILLSPEED * MAGIC),
            Element::SpeedW => (0.0, -DOWNHILLSPEED),
            Element::SpeedNW => (-DOWNHILLSPEED * MAGIC, -DOWNHILLSPEED * MAGIC),
            _ => (0.0, 0.0),
        };
        Vector2D::new(x, y)
    }

    pub fn get_friction(&self) -> f32 {
        match self {
            Element::Grass => 0.9935,
            Element::Dirt => 0.92,
            Element::Mud => 0.8,
            Element::Ice => 0.9975,
            Element::SpeedN
            | Element::SpeedNE
            | Element::SpeedS
            | Element::SpeedSE
            | Element::SpeedE
            | Element::SpeedSW
            | Element::SpeedW
            | Element::SpeedNW => 0.9935,
            Element::Water | Element::Acid => 0.0,
            Element::WaterSwamp | Element::AcidSwamp => 0.95,
            Element::Block | Element::StickyBlock | Element::BouncyBlock => 0.9,
            Element::FakeBlock => 0.9935,
            Element::OnewayN | Element::OnewayE | Element::OnewayS | Element::OnewayW => 0.995,
        }
    }
}

impl Tile {
    pub fn new(
        special: Option<Special>,
        shape: Option<Shape>,
        background: Element,
        foreground: Element,
    ) -> Self {
        Self {
            special,
            shape,
            background,
            foreground,
        }
    }

    pub fn from_i32s(
        special_value: i32,
        shape_value: i32,
        background_value: i32,
        foreground_value: i32,
    ) -> Result<Self, TileCreationError> {
        let special_parse: SpecialParse = FromPrimitive::from_i32(special_value)
            .ok_or_else(|| TileCreationError::InvalidSpecial(special_value))?;
        let background = FromPrimitive::from_i32(background_value)
            .ok_or_else(|| TileCreationError::InvalidBackground(background_value))?;
        let foreground = FromPrimitive::from_i32(foreground_value)
            .ok_or_else(|| TileCreationError::InvalidForeground(foreground_value))?;

        let (special, shape) = match special_parse {
            SpecialParse::Normal => (
                None,
                Some(
                    FromPrimitive::from_i32(shape_value)
                        .ok_or_else(|| TileCreationError::InvalidShape(shape_value))?,
                ),
            ),
            _ => (
                Some(
                    FromPrimitive::from_i32(shape_value)
                        .ok_or_else(|| TileCreationError::InvalidShape(shape_value))?,
                ),
                None,
            ),
        };

        Ok(Tile {
            special,
            shape,
            background,
            foreground,
        })
    }

    pub fn from_tile_code(tile_code: i32) -> Result<Self, TileCreationError> {
        let special_value = tile_code >> 24;
        let shape_value = (tile_code >> 16) % 256;
        let background_value = (tile_code >> 8) % 256;
        let foreground_value = tile_code % 256;

        let special_parse: SpecialParse = FromPrimitive::from_i32(special_value)
            .ok_or_else(|| TileCreationError::InvalidSpecial(special_value))?;
        let background = FromPrimitive::from_i32(background_value)
            .ok_or_else(|| TileCreationError::InvalidBackground(background_value))?;
        let foreground = FromPrimitive::from_i32(foreground_value)
            .ok_or_else(|| TileCreationError::InvalidForeground(foreground_value))?;

        let (special, shape) = match special_parse {
            SpecialParse::Normal => (
                None,
                Some(
                    FromPrimitive::from_i32(shape_value)
                        .ok_or_else(|| TileCreationError::InvalidShape(shape_value))?,
                ),
            ),
            _ => (
                Some(
                    FromPrimitive::from_i32(shape_value)
                        .ok_or_else(|| TileCreationError::InvalidShape(shape_value))?,
                ),
                None,
            ),
        };

        Ok(Tile {
            special,
            shape,
            background,
            foreground,
        })
    }

    pub fn to_tile_code(&self) -> i32 {
        let (special, shape) = match self.special {
            None => (SpecialParse::Normal as i32, self.shape.unwrap() as i32),
            _ => (SpecialParse::Special as i32, self.special.unwrap() as i32),
        };
        let background = self.background as i32;
        let foreground = self.foreground as i32;

        (special << 24) | (shape << 16) | (background << 8) | (foreground)
    }
}

impl Into<i32> for Tile {
    fn into(self) -> i32 {
        self.to_tile_code()
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            special: None,
            shape: Some(Shape::Blank),
            background: Element::Grass,
            foreground: Element::Grass,
        }
    }
}
/*
impl From<i32> for Tile {
    fn from(i: i32) -> Self {}
}
*/
