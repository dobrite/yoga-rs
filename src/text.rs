use yoga_wrapper::{Context, ContextFactory, Node};

pub struct TextFactory<'meas> {
    factory: ContextFactory<'meas>,
}

impl<'meas> TextFactory<'meas> {
    pub fn new(factory: ContextFactory) -> TextFactory {
        TextFactory { factory: factory }
    }

    pub fn create<'text>(&self, text: &'text str) -> Text<'text, 'meas> {
        Text {
            node: Node::new(),
            context: self.factory.create(text),
        }
    }
}

pub struct Text<'text, 'meas> {
    node: Node,
    context: Context<'text, 'meas>,
}

#[cfg(test)]
mod tests {
    use TextFactory;
    use yoga_wrapper;

    struct Measurer {}

    impl yoga_wrapper::Measures for Measurer {
        fn measure(&self, text: &str) -> yoga_wrapper::Size {
            yoga_wrapper::Size {
                width: text.len() as f32,
                height: 1.0,
            }
        }
    }

    #[test]
    fn it_works() {
        let m = &Measurer {};
        let tf = TextFactory::new(yoga_wrapper::ContextFactory::new(m));
        tf.create("Yo!");
    }
}
