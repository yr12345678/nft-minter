use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct FrameStraight;

impl Layer for FrameStraight {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly set stroke width
        let valid_stroke_widths = [100]; // Should be divisable by 2
        let stroke_width = valid_stroke_widths
            .get(random.roll::<usize>(1))
            .expect("Did not find a valid stroke width. This should never happen.");

        // Generate the rectangle that will be our background
        let mut rectangle = Rectangle::new()
            .set("stroke-width", *stroke_width)
            .set("fill", "none")
            .set("x", *stroke_width / 2)
            .set("y", *stroke_width / 2)
            .set("width", 1000 - *stroke_width)
            .set("height", 1000 - *stroke_width);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a darker solid color
            let color = if base_color.is_some() {
                let unwrapped = base_color.unwrap();

                HSL {
                    lightness: unwrapped.lightness - 30,
                    ..unwrapped
                }
                .as_string()
            } else {
                // Pick a random color, but prefer vibrant
                if random.roll::<u8>(100) < 30 {
                    HSL::new_random(random, ColorMode::Light, 100).as_string()
                } else {
                    HSL::new_random(random, ColorMode::Vibrant, 100).as_string()
                }
            };

            rectangle = rectangle.set("stroke", color);

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

            rectangle = rectangle.set("stroke", format!("url(#{gradient_name})",));

            vec![gradient.into(), rectangle.into()]
        }
    }
}
