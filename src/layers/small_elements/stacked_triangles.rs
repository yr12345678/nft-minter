use crate::utils::{ColorMode, HSL};
use crate::{layers::Layer, utils::random_gradient_definition};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct StackedTriangles;

// TODO: add rotation angle?
impl Layer for StackedTriangles {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
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

        // Set the colors and return the result
        if random.roll::<u8>(100) < 80 {
            // Pick two solid colors
            let random_color1 = if random.roll::<u8>(100) < 50 {
                HSL::new_light_random(random).as_string()
            } else {
                HSL::new_vibrant_random(random).as_string()
            };

            // Add the fill to the triangle
            triangle1 = triangle1.set("fill", random_color1.clone());
            triangle2 = triangle2.set("fill", random_color1);

            vec![triangle1.into(), triangle2.into()]
        } else {
            // Randomize the color mode
            let color_mode = if random.roll::<u8>(100) < 50 {
                ColorMode::Light
            } else {
                ColorMode::Vibrant
            };

            // Generate two gradient definitions
            let (random_gradient, gradient_name1) =
                random_gradient_definition(random, None, &color_mode);

            // Add the fill to the triangle
            triangle1 = triangle1.set("fill", format!("url(#{gradient_name1})"));
            triangle2 = triangle2.set("fill", format!("url(#{gradient_name1})"));

            vec![random_gradient.into(), triangle1.into(), triangle2.into()]
        }
    }
}
