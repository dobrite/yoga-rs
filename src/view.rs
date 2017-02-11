use yoga_wrapper;

use renderable::Renderable;
use style::Style;

#[derive(Default)]
pub struct View<'r> {
    node: yoga_wrapper::Node,
    style: Style,
    children: Vec<&'r Renderable>, // TODO slice?
}

impl<'r> View<'r> {
    pub fn new() -> View<'r> {
        View { ..Default::default() }
    }

    pub fn set_width(&mut self, width: f32) {
        self.style.set_width(width)
    }

    pub fn set_height(&mut self, height: f32) {
        self.style.set_height(height)
    }

    pub fn set_flex_direction(&mut self, flex_direction: yoga_wrapper::FlexDirection) {
        self.style.set_flex_direction(flex_direction)
    }

    pub fn set_padding(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.style.set_padding(edge, value)
    }

    pub fn set_margin(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.style.set_margin(edge, value)
    }

    pub fn insert_child(&mut self, child: &'r Renderable, index: usize) {
        self.style.insert_child(child.get_node(), index);
        self.children.insert(index, child)
    }
}

impl<'r> Renderable for View<'r> {
    fn get_layout_width(&self) -> f32 {
        self.style.get_layout_width()
    }

    fn get_layout_height(&self) -> f32 {
        self.style.get_layout_height()
    }

    fn get_layout_top(&self) -> f32 {
        self.style.get_layout_top()
    }

    fn get_layout_left(&self) -> f32 {
        self.style.get_layout_left()
    }

    fn calculate_layout(&mut self) {
        self.style.calculate_layout()
    }

    fn get_child_count(&self) -> usize {
        self.children.len()
    }

    fn get_child(&self, index: usize) -> Option<&Renderable> {
        match self.children.get(index) {
            Some(r) => Some(*r),
            None => None,
        }
    }

    fn get_node(&self) -> &yoga_wrapper::Node {
        &self.style.get_node()
    }
}

#[cfg(test)]
mod tests {
    use View;

    #[test]
    fn it_works() {
        let _ = View::new();
    }
}
