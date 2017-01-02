use yoga_wrapper;

pub struct ViewFactory {}

impl ViewFactory {
    pub fn new() -> ViewFactory {
        ViewFactory {}
    }

    pub fn create(&self) -> View {
        View { node: yoga_wrapper::Node::new() }
    }
}

pub struct View {
    node: yoga_wrapper::Node,
}

#[cfg(test)]
mod tests {
    use ViewFactory;

    #[test]
    fn it_works() {
        let vf = ViewFactory::new();
        vf.create();
    }
}
