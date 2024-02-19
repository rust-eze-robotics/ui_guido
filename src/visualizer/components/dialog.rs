use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{
        Color, DrawMode, FillOptions, GraphicsContext, Mesh, MeshBuilder, Rect, StrokeOptions,
        Text, TextFragment,
    },
};

use super::Component;

/// The dialog component implements Component and displays a dialog box with a text.
pub(in crate::visualizer) struct DialogComponent {
    mesh: Mesh,
    text: Text,
}

/// The struct contains the origin of the dialog component.
pub(in crate::visualizer) struct DialogComponentParam {
    origin: Vec2,
}

/// The struct contains the text to be updated in the dialog component.
pub(in crate::visualizer) struct DialogComponentUpdateParam {
    text: String,
}

impl DialogComponent {
    /// The constructor creates a new instance of the dialog component.
    pub(crate) fn new(gfx: &impl Has<GraphicsContext>, text: String) -> Self {
        // Builds background mesh
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder
            .rectangle(
                DrawMode::Fill(FillOptions::default()),
                Rect::new(0.0, 0.0, 900.0, 100.0),
                Color::from_rgba_u32(0x000080AA), // It has a bit of transparency
            )
            .unwrap();
        mesh_builder
            .rectangle(
                DrawMode::Stroke(StrokeOptions::default().with_line_width(5.0)),
                Rect::new(0.0, 0.0, 900.0, 100.0),
                Color::from_rgba_u32(0x000051FF),
            )
            .unwrap();

        // Creates the mesh
        let mesh = Mesh::from_data(gfx, mesh_builder.build());

        // The font has been loaded previously in the main file
        let mut text = Text::new(
            TextFragment::new(text)
                .font("kode")
                .color(Color::WHITE)
                .scale(40.0),
        );

        // Sets the text bounds and wrap
        text.set_bounds(vec2(860.0, 60.0));
        text.set_wrap(true);

        Self { mesh, text }
    }
}

impl Component<DialogComponentParam, DialogComponentUpdateParam> for DialogComponent {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        draw_param: ggez::graphics::DrawParam,
        component_param: DialogComponentParam,
    ) -> Result<(), ggez::GameError> {
        // Draws background mesh
        canvas.draw(
            &self.mesh,
            draw_param
                .clone()
                .dest(component_param.origin + vec2(10.0, 10.0)),
        );

        // Draws dialog text
        canvas.draw(
            &self.text,
            draw_param
                .clone()
                .dest(component_param.origin + vec2(30.0, 20.0)),
        );

        Ok(())
    }

    fn update(&mut self, update_param: DialogComponentUpdateParam) -> Result<(), ggez::GameError> {
        // Gets the only existing fragment and updates its text
        self.text.fragments_mut().get_mut(0).unwrap().text = update_param.text;

        Ok(())
    }
}

impl DialogComponentParam {
    /// The constructor creates a new instance of the dialog component parameter.
    pub(crate) fn new(origin: Vec2) -> Self {
        Self { origin }
    }
}

impl DialogComponentUpdateParam {
    /// The constructor creates a new instance of the dialog component update parameter.
    pub(crate) fn new(text: String) -> Self {
        Self { text }
    }
}
