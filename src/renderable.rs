use yoga_wrapper;
use style::{BackgroundColor, Style};

pub trait Renderable<C> {
    fn get_style(&self) -> &Style<C>;
    fn get_mut_style(&mut self) -> &mut Style<C>;
    fn get_child(&self, index: usize) -> Option<&Renderable<C>>;
    fn get_child_count(&self) -> usize;
    fn get_node(&self) -> &yoga_wrapper::Node;

    fn get_layout_width(&self) -> f32 {
        self.get_style().get_layout_width()
    }

    fn get_layout_height(&self) -> f32 {
        self.get_style().get_layout_height()
    }

    fn get_layout_top(&self) -> f32 {
        self.get_style().get_layout_top()
    }

    fn get_layout_left(&self) -> f32 {
        self.get_style().get_layout_left()
    }

    fn calculate_layout(&mut self) {
        self.get_mut_style().calculate_layout()
    }

    fn get_color(&self) -> &Option<C> {
        self.get_style().get_color()
    }

    fn get_background_color(&self) -> &Option<BackgroundColor<C>> {
        self.get_style().get_background_color()
    }

    fn set_width(&mut self, width: f32) {
        self.get_mut_style().set_width(width)
    }

    fn set_height(&mut self, height: f32) {
        self.get_mut_style().set_height(height)
    }

    fn set_align_self(&mut self, value: yoga_wrapper::Align) {
        self.get_mut_style().set_align_self(value)
    }

    fn set_flex_grow(&mut self, value: f32) {
        self.get_mut_style().set_flex_grow(value)
    }

    fn set_flex_direction(&mut self, flex_direction: yoga_wrapper::FlexDirection) {
        self.get_mut_style().set_flex_direction(flex_direction)
    }

    fn set_padding(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.get_mut_style().set_padding(edge, value)
    }

    fn set_margin(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.get_mut_style().set_margin(edge, value)
    }
}
