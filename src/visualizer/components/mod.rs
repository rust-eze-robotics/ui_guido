use ggez::graphics::{Canvas, DrawParam, InstanceArray};

pub(super) mod contents_map;
pub(super) mod dialog;
pub(super) mod player;
pub(super) mod tails_map;

/// A component that can be drawn on a canvas.
/// It has basic draw and update methods callable from the visualizer.
pub(super) trait Component<ComponentParam, UpdateParam> {
    /// Draw the component on the given canvas.
    fn draw(
        &self,
        canvas: &mut Canvas,
        draw_param: DrawParam,
        component_param: ComponentParam,
    ) -> Result<(), ggez::GameError>;

    /// Update the component.
    fn update(&mut self, _update_param: UpdateParam) -> Result<(), ggez::GameError> {
        Ok(())
    }
}

/// The struct implements the draw params position tracing.
/// It is used for adding, updating and removing elements from the instance array, which
/// doesn't provide a method for these operations.
/// A set of coordinates from elements has the same position in the vector of its
/// corresponding draw param into the instance array.
pub(self) struct CoordinatedInstance {
    array: InstanceArray,
    elements: Vec<(usize, usize)>,
}
