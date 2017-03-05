use yoga_wrapper;

use renderable::Renderable;
use style::Style;

pub struct View<'r, C: 'r> {
    style: Style<C>,
    children: Vec<&'r Renderable<C>>, // TODO slice?
}

impl<'r, C> View<'r, C> {
    pub fn new() -> View<'r, C> {
        View {
            style: Style::new(),
            children: Default::default(),
        }
    }

    pub fn insert_child(&mut self, child: &'r Renderable<C>, index: usize) {
        self.style.insert_child(child.get_node(), index);
        self.children.insert(index, child)
    }
}

impl<'r, C> Renderable<C> for View<'r, C> {
    fn get_text(&self) -> Option<&str> {
        None
    }

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
