use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementRoundedRectangle;

impl Layer for SmallElementRoundedRectangle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the required values for building the rectangle. It will vary in size
        // and we have to adjust its position and corner radius with it.
        let random_dimension = random.in_range::<u16>(75, 125) * 2; 
        let rx = random_dimension / 5; // This will just get rounded, which is fine
        let position = 500 - (random_dimension / 2);

        // Build the rectangle
        let mut rectangle = Rectangle::new()
            .set("width", random_dimension)
            .set("height", random_dimension)
            .set("rx", rx)
            .set("x", position)
            .set("y", position);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color, but prefer vibrant
                if random.roll::<u8>(100) < 30 {
                    HSL::new_random(random, ColorMode::Light, 100).as_string()
                } else {
                    HSL::new_random(random, ColorMode::Vibrant, 100).as_string()
                }
            };

            rectangle = rectangle.set("fill", color);

            vec![rectangle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Randomize the color mode, but prefer vibrant
                if random.roll::<u8>(100) < 30 {
                    random_gradient_definition(random, Some(45), ColorMode::Light, 100)
                } else {
                    random_gradient_definition(random, Some(45), ColorMode::Vibrant, 100)
                }
            };

            rectangle = rectangle.set("fill", format!("url(#{gradient_name})",));

            vec![gradient.into(), rectangle.into()]
        }
    }
}
