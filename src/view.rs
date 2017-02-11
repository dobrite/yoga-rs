use yoga_wrapper;

use renderable::Renderable;
use style::{BackgroundColor, Style};

#[derive(Default)]
pub struct View<'r, C: 'r> {
    node: yoga_wrapper::Node,
    style: Style<C>,
    children: Vec<&'r Renderable<C>>, // TODO slice?
}

impl<'r, C> View<'r, C> {
    pub fn new() -> View<'r, C>
        where C: Default
    {
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

    pub fn insert_child(&mut self, child: &'r Renderable<C>, index: usize) {
        self.style.insert_child(child.get_node(), index);
        self.children.insert(index, child)
    }
}

impl<'r, C> Renderable<C> for View<'r, C> {
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

    fn get_child(&self, index: usize) -> Option<&Renderable<C>> {
        match self.children.get(index) {
            Some(r) => Some(*r),
            None => None,
        }
    }

    fn get_color(&self) -> &Option<C> {
        self.style.get_color()
    }

    fn get_background_color(&self) -> &Option<BackgroundColor<C>> {
        self.style.get_background_color()
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
        let _: View<i32> = View::new();
    }
}
