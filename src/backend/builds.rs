use yoga_wrapper;

use View;
use Text;

pub trait Builds<'meas, C> {
    fn create_context<'text>(&'meas self,
                             text: &'text str)
                             -> Box<yoga_wrapper::Context<'text, 'meas>>;
    fn view<'r>(&self) -> View<'r, C>;
    fn text<'text>(&'meas self, text: &'text str) -> Text<'text, 'meas, C>;
}
