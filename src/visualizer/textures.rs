use ggez::{
    context::Has,
    graphics::{GraphicsContext, Image},
};
use robotics_lib::world::tile::{Content, Tile, TileType};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Texture {
    SandBlock,
    SandHalf,
    HillBlock,
    HillHalf,
    SnowBlock,
    SnowHalf,
    LavaBlock,
    LavaHalf,
    WallBlock,
    WallHalf,
    GrassBlock,
    GrassHalf,
    StreetBlock,
    StreetHalf,
    MountainBlock,
    MountainHalf,
    DeepWaterBlock,
    DeepWaterHalf,
    ShallowWaterBlock,
    ShallowWaterHalf,
    TeleportBlock,
    TeleportHalf,
    Fish,
    Tree,
    Rock,
    Fire,
    Garbage,
    Coin,
    Bin,
    Crate,
    Market,
    Player,
}

impl Texture {
    pub fn from_tile(tile: &Tile) -> Self {
        match tile.tile_type {
            TileType::Sand => {
                if tile.elevation < 3 {
                    Texture::SandHalf
                } else {
                    Texture::SandBlock
                }
            }
            TileType::Hill => {
                if tile.elevation < 3 {
                    Texture::HillHalf
                } else {
                    Texture::HillBlock
                }
            }
            TileType::Snow => {
                if tile.elevation < 3 {
                    Texture::SnowHalf
                } else {
                    Texture::SnowBlock
                }
            }
            TileType::Lava => {
                if tile.elevation < 3 {
                    Texture::LavaHalf
                } else {
                    Texture::LavaBlock
                }
            }
            TileType::Wall => {
                if tile.elevation < 3 {
                    Texture::WallHalf
                } else {
                    Texture::WallBlock
                }
            }
            TileType::Grass => {
                if tile.elevation < 3 {
                    Texture::GrassHalf
                } else {
                    Texture::GrassBlock
                }
            }
            TileType::Street => {
                if tile.elevation < 3 {
                    Texture::StreetHalf
                } else {
                    Texture::StreetBlock
                }
            }
            TileType::Mountain => {
                if tile.elevation < 3 {
                    Texture::MountainHalf
                } else {
                    Texture::MountainBlock
                }
            }
            TileType::DeepWater => {
                if tile.elevation < 3 {
                    Texture::DeepWaterHalf
                } else {
                    Texture::DeepWaterBlock
                }
            }
            TileType::ShallowWater => {
                if tile.elevation < 3 {
                    Texture::ShallowWaterHalf
                } else {
                    Texture::ShallowWaterBlock
                }
            }
            TileType::Teleport(_) => {
                if tile.elevation < 3 {
                    Texture::TeleportHalf
                } else {
                    Texture::TeleportBlock
                }
            }
        }
    }

    pub fn from_content(content: &Content) -> Option<Self> {
        match content {
            Content::None => None,
            Content::Fish(_) => Some(Texture::Fish),
            Content::Tree(_) => Some(Texture::Tree),
            Content::Rock(_) => Some(Texture::Rock),
            Content::Fire => Some(Texture::Fire),
            Content::Garbage(_) => Some(Texture::Garbage),
            Content::Coin(_) => Some(Texture::Coin),
            Content::Bin(_) => Some(Texture::Bin),
            Content::Crate(_) => Some(Texture::Crate),
            Content::Market(_) => Some(Texture::Market),
            _ => None,
        }
    }

    pub fn get_image(&self, gfx: &impl Has<GraphicsContext>) -> Image {
        match self {
            Texture::SandBlock => Image::from_path(gfx, "/blocks/sand.png").unwrap(),
            Texture::SandHalf => Image::from_path(gfx, "/halfs/sand.png").unwrap(),
            Texture::HillBlock => Image::from_path(gfx, "/blocks/hill.png").unwrap(),
            Texture::HillHalf => Image::from_path(gfx, "/halfs/hill.png").unwrap(),
            Texture::SnowBlock => Image::from_path(gfx, "/blocks/snow.png").unwrap(),
            Texture::SnowHalf => Image::from_path(gfx, "/halfs/snow.png").unwrap(),
            Texture::LavaBlock => Image::from_path(gfx, "/blocks/lava.png").unwrap(),
            Texture::LavaHalf => Image::from_path(gfx, "/halfs/lava.png").unwrap(),
            Texture::WallBlock => Image::from_path(gfx, "/blocks/wall.png").unwrap(),
            Texture::WallHalf => Image::from_path(gfx, "/halfs/wall.png").unwrap(),
            Texture::GrassBlock => Image::from_path(gfx, "/blocks/grass.png").unwrap(),
            Texture::GrassHalf => Image::from_path(gfx, "/halfs/grass.png").unwrap(),
            Texture::StreetBlock => Image::from_path(gfx, "/blocks/street.png").unwrap(),
            Texture::StreetHalf => Image::from_path(gfx, "/halfs/street.png").unwrap(),
            Texture::MountainBlock => Image::from_path(gfx, "/blocks/mountain.png").unwrap(),
            Texture::MountainHalf => Image::from_path(gfx, "/halfs/mountain.png").unwrap(),
            Texture::DeepWaterBlock => Image::from_path(gfx, "/blocks/deepwater.png").unwrap(),
            Texture::DeepWaterHalf => Image::from_path(gfx, "/halfs/deepwater.png").unwrap(),
            Texture::ShallowWaterBlock => {
                Image::from_path(gfx, "/blocks/shallowwater.png").unwrap()
            }
            Texture::ShallowWaterHalf => Image::from_path(gfx, "/halfs/shallowwater.png").unwrap(),
            Texture::TeleportBlock => Image::from_path(gfx, "/blocks/teleport.png").unwrap(),
            Texture::TeleportHalf => Image::from_path(gfx, "/halfs/teleport.png").unwrap(),
            Texture::Fish => Image::from_path(gfx, "/contents/fish.png").unwrap(),
            Texture::Tree => Image::from_path(gfx, "/contents/tree.png").unwrap(),
            Texture::Rock => Image::from_path(gfx, "/contents/rock.png").unwrap(),
            Texture::Fire => Image::from_path(gfx, "/contents/fire.png").unwrap(),
            Texture::Garbage => Image::from_path(gfx, "/contents/garbage.png").unwrap(),
            Texture::Coin => Image::from_path(gfx, "/contents/coin.png").unwrap(),
            Texture::Bin => Image::from_path(gfx, "/contents/bin.png").unwrap(),
            Texture::Crate => Image::from_path(gfx, "/contents/crate.png").unwrap(),
            Texture::Market => Image::from_path(gfx, "/contents/market.png").unwrap(),
            Texture::Player => Image::from_path(gfx, "/player/robot.png").unwrap(),
        }
    }

    pub fn width(&self) -> f32 {
        16.0
    }

    pub fn height(&self) -> f32 {
        16.0
    }
}
