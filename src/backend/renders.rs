use renderable::Renderable;

pub trait Renders {
    type Color;

    fn render(&mut self, node: &Renderable<Self::Color>);
}
