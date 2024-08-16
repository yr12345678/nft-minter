use crate::{layers::Layer, utils::HSL};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SolidBackground;

impl Layer for SolidBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Randomize the color, but prefer vibrant
        let random_color = if random.roll::<u8>(100) < 10 {
            HSL::new_light_random(random).as_string()
        } else {
            HSL::new_vibrant_random(random).as_string()
        };

        // Generate the rectangle that will be our background
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", random_color)
            .into();

        vec![background]
    }
}
