use ggez::{
    graphics::{Canvas, DrawParam},
    Context,
};

pub(super) mod contents_map;
pub(super) mod tails_map;
pub(super) mod player;

pub(crate) trait Component<ComponentParam, UpdateParam> {

    fn update(
        &mut self,
        update_param: UpdateParam
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut Canvas,
        draw_param: DrawParam,
        component_param: ComponentParam,
    ) -> Result<(), ggez::GameError>;
}
