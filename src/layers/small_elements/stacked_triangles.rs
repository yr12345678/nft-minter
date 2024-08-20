use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct StackedTriangles;

// TODO: add rotation angle?
impl Layer for StackedTriangles {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate stacked triangles with a random positioning
        let (mut triangle1, mut triangle2) = match random.roll::<u8>(8) {
            0 => {
                // From middle to top
                (
                    Polygon::new().set("points", "250,500 750,500 500,250"),
                    Polygon::new().set("points", "250,250 750,250 500,0"),
                )
            }
            1 => {
                // From middle to right
                (
                    Polygon::new().set("points", "1000,750 1000,250 750,500"),
                    Polygon::new().set("points", "750,750 750,250 500,500"),
                )
            }
            2 => {
                // From middle to bottom
                (
                    Polygon::new().set("points", "250,500 750,500 500,750"),
                    Polygon::new().set("points", "250,750 750,750 500,1000"),
                )
            }
            3 => {
                // From middle to left
                (
                    Polygon::new().set("points", "500,750 500,250 250,500"),
                    Polygon::new().set("points", "250,750 250,250 0,500"),
                )
            }
            4 => {
                // From top to middle
                (
                    Polygon::new().set("points", "250,0 750,0 500,250"),
                    Polygon::new().set("points", "250,250 750,250 500,500"),
                )
            }
            5 => {
                // From right to middle
                (
                    Polygon::new().set("points", "1000,750 1000,250 750,500"),
                    Polygon::new().set("points", "750,750 750,250 500,500"),
                )
            }
            6 => {
                // From bottom to middle
                (
                    Polygon::new().set("points", "250,1000 750,1000 500,750"),
                    Polygon::new().set("points", "250,750 750,750 500,500"),
                )
            }
            7 => {
                // From left to middle
                (
                    Polygon::new().set("points", "0,250 0,750 250,500"),
                    Polygon::new().set("points", "250,250 250,750 500,500"),
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
            triangle1 = triangle1.set("fill", color.clone());
            triangle2 = triangle2.set("fill", color);

            vec![triangle1.into(), triangle2.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Randomize the color mode, but prefer vibrant
                let color_mode = if random.roll::<u8>(100) < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            triangle1 = triangle1.set("fill", format!("url(#{gradient_name})"));
            triangle2 = triangle2.set("fill", format!("url(#{gradient_name})"));

            vec![gradient.into(), triangle1.into(), triangle2.into()]
        }
    }
}
