use View;
use Text;

pub trait Builds<C> {
    fn view<'r>() -> View<'r, C>;
    fn text(text: &str) -> Text<C>;
}
