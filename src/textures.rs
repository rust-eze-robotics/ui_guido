use ggez::{
    context::Has,
    graphics::{GraphicsContext, Image},
};
use robotics_lib::world::tile::{Tile, TileType};

pub(crate) const BLOCKS_TEXTURES_DIR_PATH: &str = "/blocks";
pub(crate) const HALFS_TEXTURES_DIR_PATH: &str = "/halfs";

pub struct TileTypeTexture {
    pub block: Image,
    pub half: Image,
}

impl TileTypeTexture {
    pub fn from_tiletype(gfx: &impl Has<GraphicsContext>, tiletype: &TileType) -> TileTypeTexture {
        match tiletype {
            TileType::Sand => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/sand.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/sand.png"))
                    .unwrap(),
            },
            TileType::Hill => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/hill.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/hill.png"))
                    .unwrap(),
            },
            TileType::Snow => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/snow.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/snow.png"))
                    .unwrap(),
            },
            TileType::Lava => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/lava.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/lava.png"))
                    .unwrap(),
            },
            TileType::Wall => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/wall.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/wall.png"))
                    .unwrap(),
            },
            TileType::Grass => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/grass.png"),
                )
                .unwrap(),
                half: Image::from_path(gfx, format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/grass.png"))
                    .unwrap(),
            },
            TileType::Street => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/street.png"),
                )
                .unwrap(),
                half: Image::from_path(
                    gfx,
                    format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/street.png"),
                )
                .unwrap(),
            },
            TileType::Mountain => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/mountain.png"),
                )
                .unwrap(),
                half: Image::from_path(
                    gfx,
                    format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/mountain.png"),
                )
                .unwrap(),
            },
            TileType::DeepWater => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/deepwater.png"),
                )
                .unwrap(),
                half: Image::from_path(
                    gfx,
                    format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/deepwater.png"),
                )
                .unwrap(),
            },
            TileType::ShallowWater => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/shallowwater.png"),
                )
                .unwrap(),
                half: Image::from_path(
                    gfx,
                    format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/shallowwater.png"),
                )
                .unwrap(),
            },
            TileType::Teleport(_) => TileTypeTexture {
                block: Image::from_path(
                    gfx,
                    format!("{}{}", BLOCKS_TEXTURES_DIR_PATH, "/teleport.png"),
                )
                .unwrap(),
                half: Image::from_path(
                    gfx,
                    format!("{}{}", HALFS_TEXTURES_DIR_PATH, "/teleport.png"),
                )
                .unwrap(),
            },
        }
    }
}

pub fn load_tiles_matrix_textures(
    gfx: &impl Has<GraphicsContext>,
    matrix: &Vec<Vec<Tile>>,
) -> Vec<Vec<Image>> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| {
                    let texture = TileTypeTexture::from_tiletype(gfx, &tile.tile_type);
                    if tile.elevation < 2 {
                        texture.half
                    } else {
                        texture.block
                    }
                })
                .collect()
        })
        .collect()
}
