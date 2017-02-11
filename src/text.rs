use yoga_wrapper;

use renderable::Renderable;
use style::{BackgroundColor, Style};

#[derive(Debug, Default)]
pub struct Text<'text, C> {
    text: &'text str,
    style: Style<C>,
}

impl<'text, C> Text<'text, C> {
    pub fn new(text: &'text str) -> Text<'text, C>
        where C: Default
    {
        Text {
            text: text,
            style: Style::new(),
        }
    }
}

impl<'text, C> Renderable<C> for Text<'text, C> {
    fn get_node(&self) -> &yoga_wrapper::Node {
        self.style.get_node()
    }

    fn get_style(&self) -> &Style<C> {
        &self.style
    }

    fn get_mut_style(&mut self) -> &mut Style<C> {
        &mut self.style
    }

    fn get_child(&self, _: usize) -> Option<&Renderable<C>> {
        None
    }

    fn get_child_count(&self) -> usize {
        0
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


    impl Renders for Renderer {
        type Color = i32;
        fn render(&mut self, node: &Renderable<i32>) {}
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
        type Renderer = Renderer;

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
        let _: Text<i32> = Text::new("yo!");
    }
}
