use crate::hsl::*;
use crate::{layers::Layer, utils::random_gradient_definition};
use random::Random;
use svg::node::element::{Circle, Element};

pub struct FullCircle;

impl Layer for FullCircle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Pick a random radius to introduce some variety
        let random_radius = random.in_range::<u16>(150, 250) * 2; // Always an even number

        // Create our circle
        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            // Pick a solid color
            let random_color = if random.roll::<u8>(100) < 50 {
                HSL::new_random(random, ColorMode::Light, 100).as_string()
            } else {
                HSL::new_random(random, ColorMode::Vibrant, 100).as_string()
            };
            circle = circle.set("fill", random_color);

            vec![circle.into()]
        } else {
            // Get a gradient definition and name and add it as a fill to the path
            let (random_gradient, gradient_name) = if random.roll::<u8>(100) < 50 {
                random_gradient_definition(random, None, ColorMode::Light, 100)
            } else {
                random_gradient_definition(random, None, ColorMode::Vibrant, 100)
            };

            circle = circle.set("fill", format!("url(#{gradient_name})",));

            vec![random_gradient.into(), circle.into()]
        }
    }
}
