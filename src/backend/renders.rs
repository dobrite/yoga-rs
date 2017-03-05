use backend::Builds;
use renderable::Renderable;

pub trait Renders<'meas> {
    type Color;
    type Input;
    type Output;
    type Builder: Builds<'meas, Self::Color>;

    fn render(&mut self, node: &Renderable<Self::Color>, input: &mut Self::Input) -> Self::Output;
}
