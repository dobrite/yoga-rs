use yoga_wrapper;

use renderable::Renderable;
use style::{BackgroundColor, Style};

#[derive(Debug)]
pub struct Text<'text, 'meas, C> {
    text: &'text str, // TODO this needs owned?
    style: Style<C>,
    context: Box<yoga_wrapper::Context<'text, 'meas>>,
}

impl<'text, 'meas, C> Text<'text, 'meas, C> {
    pub fn new(text: &'text str,
               mut context: Box<yoga_wrapper::Context<'text, 'meas>>)
               -> Text<'text, 'meas, C>
        where C: Default
    {
        let mut style = Style::new();
        style.set_measure_func(yoga_wrapper::measure);
        style.set_context(&mut context);

        Text {
            text: text,
            style: style,
            context: context,
        }
    }
}

impl<'text, 'meas, C> Renderable<C> for Text<'text, 'meas, C> {
    fn get_text(&self) -> Option<&str> {
        Some(self.text)
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
    use View;
    use Backend;
    use Builds;
    use Renders;
    use Renderable;

    struct Builder {
        measurer: Measurer,
    }

    impl<'meas> Builds<'meas, i32> for Builder {
        fn create_context<'text>(&'meas self,
                                 text: &'text str)
                                 -> Box<yoga_wrapper::Context<'text, 'meas>> {
            Box::new(yoga_wrapper::Context::new(text, &self.measurer))
        }

        fn view<'r>(&self) -> View<'r, i32> {
            View::new()
        }

        fn text<'text>(&'meas self, text: &'text str) -> Text<'text, 'meas, i32> {
            Text::new(text, self.create_context(text))
        }
    }

    struct Renderer {}

    impl<'meas> Renders<'meas> for Renderer {
        type Color = i32;
        type Builder = Builder;

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
    }

    #[test]
    fn it_works() {
        (Builder { measurer: Measurer {} }).text("yo!!");
    }

    #[test]
    fn it_calculates_layout() {
        let builder = Builder { measurer: Measurer {} };
        let mut text = builder.text("yo!!");
        text.calculate_layout();
    }
}
