use yoga_wrapper;
use renderable::Renderable;

use super::renders::Renders;

pub trait Backend<'meas> {
    type Renderer: Renders;

    fn get_renderer(&mut self) -> &mut Self::Renderer;
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas>;

    fn render(&mut self, node: &Renderable<<<Self as Backend<'meas>>::Renderer as Renders>::Color>) {
        self.get_renderer().render(node)
    }
}
