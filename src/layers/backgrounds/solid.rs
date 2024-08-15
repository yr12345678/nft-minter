use crate::layers::Layer;
use crate::utils::random_color;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SolidBackground;

impl Layer for SolidBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate a random color
        let random_color = random_color(random);

        // Generate the rectangle that will be our background
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", random_color)
            .into();

        vec![background]
    }
}
