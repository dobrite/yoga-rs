use yoga_wrapper;

use super::renders::Renders;

pub trait Backend<'meas> {
    type Color;
    type Renderer: Renders;
    type Measurer: yoga_wrapper::Measures;

    fn render(&self, renderer: &Self::Renderer, node: &yoga_wrapper::Node);
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas>;
}
