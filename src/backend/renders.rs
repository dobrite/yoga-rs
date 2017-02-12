use backend::Builds;
use renderable::Renderable;

pub trait Renders {
    type Color;
    type Builder: Builds<Self::Color>;

    fn render(&mut self, node: &Renderable<Self::Color>);
}
