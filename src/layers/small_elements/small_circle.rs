use crate::hsl::*;
use crate::{layers::Layer, utils::random_gradient_definition};
use random::Random;
use svg::node::element::{Circle, Element};

pub struct SmallCircle;

// TODO: split up gradient and solid into separate variants?
impl Layer for SmallCircle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(50, 150) * 2; // Always an even number

        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 80 {
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
            let (random_gradient, gradient_name) = if random.roll::<u8>(100) < 10 {
                random_gradient_definition(random, None, ColorMode::Light, 100)
            } else {
                random_gradient_definition(random, None, ColorMode::Vibrant, 100)
            };

            circle = circle.set("fill", format!("url(#{gradient_name})",));

            vec![random_gradient.into(), circle.into()]
        }
    }
}
