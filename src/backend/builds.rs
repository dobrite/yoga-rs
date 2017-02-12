use yoga_wrapper;

use View;
use Text;

pub trait Builds<'meas, C> {
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas>;
    fn view<'r>(&self) -> View<'r, C>;
    fn text<'t, 'a: 't>(&'a self, text: &'t str) -> Text<'t, C>;
}
