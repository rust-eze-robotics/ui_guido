use std::{env, path::PathBuf};

use ggez::{
    event::EventHandler,
    glam::vec2,
    graphics::{Canvas, Color, DrawParam, FilterMode, Image, Rect, Sampler},
};
use midgard::{
    world_generator::{WorldGenerator, WorldGeneratorParameters},
    world_visualizer::WorldVisualizer,
};
use visualizer::Visualizer;

use robotics_lib::world::{tile::Tile, world_generator::Generator};

mod textures;
mod visualizer;

struct State {
    map: Vec<Vec<Tile>>,
    map_images: Vec<Vec<Image>>,
    // image_scale: f32,
    visualizer: Visualizer,
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.visualizer.draw(ctx)?;
        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("robotics", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Robotics"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 1200.0))
        .add_resource_path(match env::var("CARGO_MANIFEST_DIR") {
            Ok(manifest_dir) => {
                let mut path = PathBuf::from(manifest_dir);
                path.push("resources");
                path
            }
            Err(_) => PathBuf::from("./resources"),
        })
        .build()
        .unwrap();

    let params = WorldGeneratorParameters {
        world_size: 20,
        amount_of_rivers: Some(1.0),
        amount_of_streets: Some(1.0),
        amount_of_teleports: Some(1.0),
        always_sunny: true,
        ..Default::default()
    };

    let mut world_generator = WorldGenerator::new(params);
    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    // let state = State {
    //     image: textures::TileTypeTexture::from_tiletype(
    //         &ctx,
    //         &robotics_lib::world::tile::TileType::Street,
    //     )
    //     .half,
    //     map_images: textures::load_tiles_matrix_textures(&ctx, &map),
    //     map,
    //     image_scale: 4.0
    // };

    let state = State {
        map: map.clone(),
        map_images: textures::load_tiles_matrix_textures(&ctx, &map),
        visualizer: Visualizer::new(
            textures::load_tiles_matrix_textures(&ctx, &map),
            vec2(200.0, 100.0),
            4.0,
        ),
    };

    ggez::event::run(ctx, event_loop, state);
}
