use backend::Builds;
use renderable::Renderable;

pub trait Renders<'meas> {
    type Color;
    type Builder: Builds<'meas, Self::Color>;

    fn render(&mut self, node: &Renderable<Self::Color>);
}
