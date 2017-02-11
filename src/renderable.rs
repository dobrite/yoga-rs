use yoga_wrapper;

pub trait Renderable {
    fn get_layout_width(&self) -> f32;
    fn get_layout_height(&self) -> f32;
    fn get_layout_top(&self) -> f32;
    fn get_layout_left(&self) -> f32;
    fn calculate_layout(&mut self);
    fn get_child_count(&self) -> usize;
    fn get_child(&self, index: usize) -> Option<&Renderable>;
    fn get_node(&self) -> &yoga_wrapper::Node;
}
