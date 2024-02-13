pub(crate) use ggez::context::Has;
pub(crate) use ggez::graphics::GraphicsContext;
pub(crate) use ggez::graphics::Image;

use crate::visualizer::textures::Texture;

use super::Component;

pub(crate) struct PlayerComponent {
    image: Image,
}

pub(crate) struct PlayerComponentParam;
pub(crate) struct PlayerComponentUpdateParam;

impl PlayerComponent {
    pub(crate) fn new(gfx: &impl Has<GraphicsContext>) -> Self {
        Self {
            image: Texture::Player.get_image(gfx),
        }
    }

    pub(crate) fn texture() -> Texture {
        Texture::Player
    }
}

impl Component<PlayerComponentParam, PlayerComponentUpdateParam> for PlayerComponent {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        draw_param: ggez::graphics::DrawParam,
        _component_param: PlayerComponentParam,
    ) -> Result<(), ggez::GameError> {
        canvas.draw(&self.image, draw_param);
        Ok(())
    }
}
