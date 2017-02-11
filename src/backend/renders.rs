use renderable::Renderable;

pub trait Renders<R: Renderable + ?Sized> {
    fn render(&mut self, node: &R);
}
