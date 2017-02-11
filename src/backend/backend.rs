use yoga_wrapper;

use super::renders::Renders;
use renderable::Renderable;

pub trait Backend<'meas> {
    type Color;
    type Renderer: Renders<Renderable>;

    fn render(&mut self, node: &Renderable);
    fn get_renderer(&mut self) -> &mut Self::Renderer;
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas>;
}
