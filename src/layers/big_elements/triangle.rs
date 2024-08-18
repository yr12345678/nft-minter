use crate::hsl::*;
use crate::{layers::Layer, utils::random_gradient_definition};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct BigTriangle;

// TODO: add rotation angle?
impl Layer for BigTriangle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate a triangle with a random positioning
        let mut triangle = match random.roll::<u8>(8) {
            0 => Polygon::new().set("points", "0,0 500,500 0,1000"), // Base to left side
            1 => Polygon::new().set("points", "0,0 500,500 1000,0"), // Base to top
            2 => Polygon::new().set("points", "1000,0 500,500 1000,1000"), // Base to right side
            3 => Polygon::new().set("points", "0,1000 500,500 1000,1000"), // Base to bottom
            4 => Polygon::new().set("points", "500,0 0,500 500,1000"), // Point to left side
            5 => Polygon::new().set("points", "0,500 500,0 1000,500"), // Point to top
            6 => Polygon::new().set("points", "500,0 1000,500 500,1000"), // Point to right
            7 => Polygon::new().set("points", "0,500 500,1000 1000,500"), // Point to bottom
            _ => panic!("No matching triangle variant"),
        };

        // Set the colors and return the result
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let random_color1 = if random.roll::<u8>(100) < 50 {
                HSL::new_random(random, ColorMode::Light, 100).as_string()
            } else {
                HSL::new_random(random, ColorMode::Vibrant, 100).as_string()
            };

            // Add the fill to the triangle
            triangle = triangle.set("fill", random_color1);

            vec![triangle.into()]
        } else {
            // Generate a gradient definition and add it to the triangle
            let (random_gradient, gradient_name1) = if random.roll::<u8>(100) < 50 {
                random_gradient_definition(random, None, ColorMode::Light, 100)
            } else {
                random_gradient_definition(random, None, ColorMode::Vibrant, 100)
            };

            triangle = triangle.set("fill", format!("url(#{gradient_name1})"));

            vec![random_gradient.into(), triangle.into()]
        }
    }
}
