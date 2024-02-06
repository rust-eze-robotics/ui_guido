use std::{env, path::PathBuf};

use ggez::{
    event::EventHandler,
    glam::vec2,
    graphics::{
        Canvas, Color, DrawParam, FilterMode, Image, Sampler,
    },
};

mod textures;

struct State {
    image: Image,
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::MAGENTA);

        let mut sampler = Sampler::default();
        sampler.mag = FilterMode::Nearest;
        sampler.min = FilterMode::Nearest;
        canvas.set_sampler(sampler);

        canvas.draw(&self.image, DrawParam::new().scale(vec2(10.0, 10.0)));

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("robotics", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Robotics"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
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

    let state = State {
        image: textures::TileTypeTexture::from_tiletype(
            &ctx,
            &robotics_lib::world::tile::TileType::Street,
        )
        .half, // image: Image::/* from_path */(&ctx, "/blocks/sand.png").unwrap()
    };

    ggez::event::run(ctx, event_loop, state);
}
