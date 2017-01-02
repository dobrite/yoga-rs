use yoga_wrapper;

use super::renders::Renders;

pub trait Backend {
    type Color;
    type Renderer: Renders;
    type Measurer: yoga_wrapper::Measures;

    fn render(&self, renderer: &Self::Renderer, node: &yoga_wrapper::Node);
}
