use std::default::Default;

use yoga_wrapper;

use rgb::RGB;

#[derive(Debug, PartialEq)]
pub enum BackgroundColor {
    Color(RGB<u8>),
    Transparent,
}

impl Default for BackgroundColor {
    fn default() -> BackgroundColor {
        BackgroundColor::Transparent
    }
}

#[derive(Debug, Default)]
pub struct Style {
    color: Option<RGB<u8>>,
    background_color: Option<BackgroundColor>,
    node: yoga_wrapper::Node,
}

impl Style {
    pub fn new() -> Self {
        Style { ..Default::default() }
    }

    pub fn get_layout_width(&self) -> f32 {
        self.node.get_layout_width()
    }

    pub fn get_layout_height(&self) -> f32 {
        self.node.get_layout_height()
    }

    pub fn get_layout_top(&self) -> f32 {
        self.node.get_layout_top()
    }

    pub fn get_layout_left(&self) -> f32 {
        self.node.get_layout_left()
    }

    pub fn set_width(&mut self, width: f32) {
        self.node.set_width(width)
    }

    pub fn set_height(&mut self, height: f32) {
        self.node.set_height(height)
    }

    pub fn calculate_layout(&mut self) {
        self.node.calculate_layout()
    }

    pub fn set_flex_direction(&mut self, flex_direction: yoga_wrapper::FlexDirection) {
        self.node.set_flex_direction(flex_direction)
    }

    pub fn set_padding(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.node.set_padding(edge, value)
    }

    pub fn set_margin(&mut self, edge: yoga_wrapper::Edge, value: f32) {
        self.node.set_margin(edge, value)
    }

    pub fn insert_child(&mut self, child: &yoga_wrapper::Node, index: usize) {
        self.node.insert_child(child, index as u32)
    }

    pub fn get_node(&self) -> &yoga_wrapper::Node {
        &self.node
    }

    pub fn set_align_self(&mut self, value: yoga_wrapper::Align) {
        self.node.set_align_self(value)
    }

    pub fn set_flex_grow(&mut self, value: f32) {
        self.node.set_flex_grow(value)
    }
}
