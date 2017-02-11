use yoga_wrapper;

use renderable::Renderable;
use style::Style;

#[derive(Debug, Default)]
pub struct Text<'text> {
    node: yoga_wrapper::Node,
    text: &'text str,
    style: Style,
}

impl<'text> Text<'text> {
    pub fn new(text: &'text str) -> Text<'text> {
        Text { text: text, ..Default::default() }
    }

    pub fn set_width(&mut self, width: f32) {
        self.style.set_width(width)
    }

    pub fn set_height(&mut self, height: f32) {
        self.style.set_height(height)
    }

    pub fn set_align_self(&mut self, value: yoga_wrapper::Align) {
        self.style.set_align_self(value)
    }

    pub fn set_flex_grow(&mut self, value: f32) {
        self.style.set_flex_grow(value)
    }
}

impl<'text> Renderable for Text<'text> {
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
        0
    }

    fn get_child(&self, _: usize) -> Option<&Renderable> {
        None
    }

    fn get_node(&self) -> &yoga_wrapper::Node {
        &self.style.get_node()
    }
}

#[cfg(test)]
mod tests {
    use yoga_wrapper;

    use Text;
    use Backend;
    use Renders;
    use Renderable;

    struct Renderer {}


    impl<R: Renderable + ?Sized> Renders<R> for Renderer {
        fn render(&mut self, node: &R) {}
    }

    struct Measurer {}

    impl yoga_wrapper::Measures for Measurer {
        fn measure(&self, text: &str) -> yoga_wrapper::Size {
            yoga_wrapper::Size {
                width: text.len() as f32,
                height: 1.0,
            }
        }
    }

    struct TestBackend {
        renderer: Renderer,
        measurer: Measurer,
    }

    impl TestBackend {
        pub fn new() -> TestBackend {
            TestBackend {
                renderer: Renderer {},
                measurer: Measurer {},
            }
        }
    }

    impl<'meas> Backend<'meas> for TestBackend {
        type Color = i32;
        type Renderer = Renderer;

        fn render(&mut self, node: &Renderable) {}

        fn get_renderer(&mut self) -> &mut Self::Renderer {
            &mut self.renderer
        }

        fn create_context<'text>(&'meas self,
                                 text: &'text str)
                                 -> yoga_wrapper::Context<'text, 'meas> {
            yoga_wrapper::Context::new(text, &self.measurer)
        }
    }

    #[test]
    fn it_works() {
        let be = TestBackend::new();
        let _ = Text::new("yo!");
    }
}
