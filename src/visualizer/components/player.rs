use ggez::context::Has;
use ggez::glam::vec2;
use ggez::graphics::GraphicsContext;
use ggez::graphics::Image;

use crate::visualizer::textures::Texture;

use super::Component;

/// The structure contains the state of the player component.
pub(in crate::visualizer) struct PlayerComponent {
    image: Image,
    position: (usize, usize),
    map_size: (usize, usize),
}

/// The structure contains parameters required by draw function.
pub(in crate::visualizer) struct PlayerComponentParam {
    scale: f32,
}

/// The structure contains parameters required by update function.
pub(in crate::visualizer) struct PlayerComponentUpdateParam {
    position: (usize, usize),
}

impl PlayerComponent {

    /// The constructor creates a new instance of the player component.
    pub(crate) fn new(
        gfx: &impl Has<GraphicsContext>,
        initial_position: (usize, usize),
        map_size: (usize, usize),
    ) -> Self {
        Self {
            image: Texture::Player.get_image(gfx),
            position: initial_position,
            map_size,
        }
    }
}

impl Component<PlayerComponentParam, PlayerComponentUpdateParam> for PlayerComponent {

    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        draw_param: ggez::graphics::DrawParam,
        component_param: PlayerComponentParam,
    ) -> Result<(), ggez::GameError> {

        let y = self.position.0 as usize;   // row
        let x = self.position.1 as usize;   // column
        let scale = component_param.scale;

        let player_x =
            (self.image.width() as f32 * 0.5) * (self.map_size.1 as usize - y + x - 1) as f32;
        let player_y = ((Texture::height() - 1.0) * 0.25) * (x + y) as f32;

        // Draws the player component.
        canvas.draw(
            &self.image,
            draw_param
                .dest(vec2(player_x, player_y - 2.0) * scale)
                .scale(vec2(scale, scale)),
        );
        Ok(())
    }

    fn update(&mut self, update_param: PlayerComponentUpdateParam) -> Result<(), ggez::GameError> {
        
        // Updates the position of the player component.
        self.position = update_param.position;
        Ok(())
    }
}

impl PlayerComponentParam {

    /// The constructor creates a new instance of the player component parameters.
    pub(crate) fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl PlayerComponentUpdateParam {

    /// The constructor creates a new instance of the player component update parameters.
    pub(crate) fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
}
