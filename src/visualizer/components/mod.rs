use ggez::{
    graphics::{Canvas, DrawParam},
    Context,
};


pub(super) mod contents_map;
pub(super) mod tails_map;
pub(super) mod player;

pub(crate) trait Component<ComponentParam> {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut Canvas,
        draw_param: DrawParam,
        component_param: ComponentParam,
    ) -> Result<(), ggez::GameError>;
}
