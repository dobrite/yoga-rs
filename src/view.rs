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

    pub fn insert_child(&mut self, child: &'r Renderable<C>, index: usize) {
        self.style.insert_child(child.get_node(), index);
        self.children.insert(index, child)
    }
}

impl<'r, C> Renderable<C> for View<'r, C> {
    fn get_node(&self) -> &yoga_wrapper::Node {
        self.style.get_node()
    }

    fn get_style(&self) -> &Style<C> {
        &self.style
    }

    fn get_mut_style(&mut self) -> &mut Style<C> {
        &mut self.style
    }

    fn get_child(&self, index: usize) -> Option<&Renderable<C>> {
        match self.children.get(index) {
            Some(r) => Some(*r),
            None => None,
        }
    }

    fn get_child_count(&self) -> usize {
        self.children.len()
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
