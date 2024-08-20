use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use scrypto::info;
use svg::node::element::{Element, Polygon};

pub struct BigTriangle;

impl Layer for BigTriangle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate a triangle with a random positioning and appropriate gradient rotation
        let (mut triangle, rotate) = match random.roll::<u8>(8) {
            0 => {
                // Base to left side
                (Polygon::new().set("points", "0,0 500,500 0,1000"), None)
            }
            1 => {
                // Base to top
                (Polygon::new().set("points", "0,0 500,500 1000,0"), Some(90))
            }
            2 => {
                // Base to right side
                (
                    Polygon::new().set("points", "1000,0 500,500 1000,1000"),
                    None,
                )
            }
            3 => {
                // Base to bottom
                (
                    Polygon::new().set("points", "0,1000 500,500 1000,1000"),
                    Some(90),
                )
            }
            4 => {
                // Point to left side
                (Polygon::new().set("points", "500,0 0,500 500,1000"), None)
            }
            5 => {
                // Point to top
                (
                    Polygon::new().set("points", "0,500 500,0 1000,500"),
                    Some(90),
                )
            }
            6 => {
                // Point to right
                (
                    Polygon::new().set("points", "500,0 1000,500 500,1000"),
                    None,
                )
            }
            7 => {
                // Point to bottom
                (
                    Polygon::new().set("points", "0,500 500,1000 1000,500"),
                    Some(90),
                )
            }
            _ => panic!("No matching triangle variant"),
        };

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let color_mode = if random.roll::<u8>(100) < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            // Add the fill to the triangle
            triangle = triangle.set("fill", color);

            vec![triangle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, None, color1, color2)
            } else {
                // Randomize the color mode, but prefer vibrant
                let color_mode = if random.roll::<u8>(100) < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, rotate, color_mode, 100)
            };

            // Add the fill to the triangle
            triangle = triangle.set("fill", format!("url(#{gradient_name})"));

            vec![gradient.into(), triangle.into()]
        }
    }
}
