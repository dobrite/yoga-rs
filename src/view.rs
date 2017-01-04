use yoga_wrapper;

pub struct View {
    node: yoga_wrapper::Node,
}

impl View {
    pub fn new() -> View {
        View { node: yoga_wrapper::Node::new() }
    }
}

#[cfg(test)]
mod tests {
    use View;

    #[test]
    fn it_works() {
        let _ = View::new();
    }
}
