use super::renders::Renders;
use renderable::Renderable;

pub trait Backend<'meas> {
    type Renderer: Renders<'meas>;

    fn get_renderer(&mut self) -> &mut Self::Renderer;

    fn render(
        &mut self,
        node: &Renderable<<<Self as Backend<'meas>>::Renderer as Renders<'meas>>::Color>,
        input: <<Self as Backend<'meas>>::Renderer as Renders<'meas>>::Input
    ) -> <<Self as Backend<'meas>>::Renderer as Renders<'meas>>::Output {
        self.get_renderer().render(node, input)
    }
}
