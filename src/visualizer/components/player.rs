pub(crate) use ggez::context::Has;
use ggez::glam::vec2;
pub(crate) use ggez::graphics::GraphicsContext;
pub(crate) use ggez::graphics::Image;

use crate::visualizer::textures::Texture;

use super::Component;

pub(crate) struct PlayerComponent {
    image: Image,
    position: (usize, usize),
    map_size: (usize, usize),
}

pub(crate) struct PlayerComponentParam {
    pub(crate) scale: f32,
}

pub(crate) struct PlayerComponentUpdateParam {
    pub(crate) position: (usize, usize),
}

impl PlayerComponent {
    pub(crate) fn new(
        gfx: &impl Has<GraphicsContext>,
        initial_position: (usize, usize),
        map_size: (usize, usize)
    ) -> Self {
        Self {
            image: Texture::Player.get_image(gfx),
            position: initial_position,
            map_size,
        }
    }

    pub(crate) fn texture() -> Texture {
        Texture::Player
    }
    
    pub(crate) fn position(&self) -> (usize, usize) {
        self.position
    }
}

impl Component<PlayerComponentParam, PlayerComponentUpdateParam> for PlayerComponent {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        draw_param: ggez::graphics::DrawParam,
        component_param: PlayerComponentParam,
    ) -> Result<(), ggez::GameError> {
    
        let y = self.position.0 as usize;
        let x = self.position.1 as usize;
        let scale = component_param.scale;
        // let player_x = (self.image.width() as f32 * 0.5)
        //     * (map_size_y as usize - y + x - 1) as f32;
        // let player_y = ((self.image.height() as f32 - 1.0) * 0.25) * (x + y) as f32;

        // println!("PlayerComponent::draw: x: {}, y: {}", x, y);
        // println!("Map size: {:?}", self.map_size);
        // println!("Scale: {}", scale);

        let player_x = (self.image.width() as f32 * 0.5)
            * (self.map_size.1 as usize - y + x - 1) as f32;
        let player_y = ((PlayerComponent::texture().height() - 1.0) * 0.25) * (x + y) as f32;
        
        // println!("PlayerComponent::draw: player_x: {}, player_y: {}", player_x, player_y);
        canvas.draw(&self.image, draw_param
            .dest(vec2(player_x, player_y + 2.0) * scale)
            .scale(vec2(scale, scale))
        );
        Ok(())
    }

    fn update(&mut self, update_param: PlayerComponentUpdateParam) -> Result<(), ggez::GameError> {
        self.position = update_param.position;
        Ok(())
    }    
}

impl PlayerComponentParam {
    pub(crate) fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl PlayerComponentUpdateParam {
    pub(crate) fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
}
