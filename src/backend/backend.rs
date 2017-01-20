use yoga_wrapper;

use super::renders::Renders;

pub trait Backend<'meas> {
    type Color;
    type Renderer: Renders;
    type Measurer: yoga_wrapper::Measures;

    fn render(&mut self, node: &yoga_wrapper::Node);
    fn get_renderer(&mut self) -> &mut Self::Renderer;
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas>;
}
