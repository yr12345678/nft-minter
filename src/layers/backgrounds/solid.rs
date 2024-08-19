use crate::hsl::*;
use crate::layers::Layer;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SolidBackground;

impl Layer for SolidBackground {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick a color, depending on whether there's a base color
        let color = if base_color.is_some() {
            base_color.unwrap().as_string() // Since it's a solid background, we just use the base color as the background
        } else {
            // Randomize the color, but prefer vibrant
            if random.roll::<u8>(100) < 30 {
                HSL::new_random(random, ColorMode::Light, 100).as_string()
            } else {
                HSL::new_random(random, ColorMode::Vibrant, 100).as_string()
            }
        };

        // Generate the rectangle that will be our background
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", color)
            .into();

        vec![background]
    }
}
