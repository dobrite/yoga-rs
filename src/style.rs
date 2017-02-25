use std::default::Default;

use yoga_wrapper;

#[derive(Debug, PartialEq)]
pub enum BackgroundColor<C> {
    Color(C),
    Transparent,
}

impl<C> Default for BackgroundColor<C> {
    fn default() -> BackgroundColor<C> {
        BackgroundColor::Transparent
    }
}

#[derive(Debug, Default)]
pub struct Style<C> {
    color: Option<C>,
    background_color: Option<BackgroundColor<C>>,
    node: yoga_wrapper::Node,
}

impl<C> Style<C> {
    pub fn new() -> Self
        where C: Default
    {
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

    pub fn get_color(&self) -> &Option<C> {
        &self.color
    }

    pub fn get_background_color(&self) -> &Option<BackgroundColor<C>> {
        &self.background_color
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

    pub fn set_measure_func(&mut self,
                            func: extern "C" fn(*mut yoga_wrapper::RawNode,
                                                f32,
                                                yoga_wrapper::MeasureMode,
                                                f32,
                                                yoga_wrapper::MeasureMode)
                                                -> yoga_wrapper::Size) {
        self.node.set_measure_func(func);
    }

    pub fn set_context(&mut self, context: &mut yoga_wrapper::Context) {
        self.node.set_context(context)
    }

    pub fn set_align_self(&mut self, value: yoga_wrapper::Align) {
        self.node.set_align_self(value)
    }

    pub fn set_flex_grow(&mut self, value: f32) {
        self.node.set_flex_grow(value)
    }
}
