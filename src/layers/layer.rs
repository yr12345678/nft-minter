use random::Random;
use svg::node::element::Element;

pub trait Layer {
    fn generate(&self, random: &mut Random) -> Vec<Element>;
}
