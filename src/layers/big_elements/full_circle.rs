use random::Random;
use crate::{layers::Layer, utils::{random_color, random_gradient_definition, ColorMode, HSL}};
use svg::node::element::{Element, Circle};

pub struct FullCircle;

// TODO: split up gradient and solid into separate variants?
impl Layer for FullCircle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(150, 250) * 2; // Always an even number
    
        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            // Pick a solid color
            let random_color = if random.roll::<u8>(100) < 50 {
                HSL::new_light_random(random).as_string()
            } else {
                HSL::new_vibrant_random(random).as_string()
            };
            circle = circle.set("fill", random_color);

            vec![circle.into()]
        } else {
            // Randomize the color mode, but prefer vibrant
            let color_mode = if random.roll::<u8>(100) < 50 {
                ColorMode::Light
            } else {
                ColorMode::Vibrant
            };
                        
            // Get a gradient definition and name and add it as a fill to the path
            let (random_gradient, gradient_name) = random_gradient_definition(random, None, &color_mode);
            circle = circle.set("fill", format!("url(#{gradient_name})",));

            vec![random_gradient.into(), circle.into()]
        }
    }
}