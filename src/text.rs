use yoga_wrapper;

pub struct Text<'text> {
    node: yoga_wrapper::Node,
    text: &'text str,
}

impl<'text> Text<'text> {
    pub fn new(text: &'text str) -> Text<'text> {
        Text {
            node: yoga_wrapper::Node::new(),
            text: text,
        }
    }
}

#[cfg(test)]
mod tests {
    use yoga_wrapper;

    use Text;
    use Backend;
    use Renders;

    struct Renderer {}

    impl Renders for Renderer {
        fn render(&self, node: &yoga_wrapper::Node) {}
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
        type Measurer = Measurer;

        fn render(&self, node: &yoga_wrapper::Node) {}

        fn get_renderer(&self) -> &Self::Renderer {
            &self.renderer
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
