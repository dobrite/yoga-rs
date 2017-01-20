use yoga_wrapper;

pub trait Renders {
    fn render(&mut self, node: &yoga_wrapper::Node);
}
