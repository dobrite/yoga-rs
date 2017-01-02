use yoga_wrapper;

pub trait Renders {
    fn render(&self, node: &yoga_wrapper::Node);
}
