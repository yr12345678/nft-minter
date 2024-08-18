use crate::hsl::*;
use crate::{layers::Layer, utils::random_gradient_definition};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct DiagonalSplitBackground;

impl Layer for DiagonalSplitBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate the two triangles that will make up the diagonal split background
        let mut triangle1 = Polygon::new().set("points", "0, 0, 1000, 1000, 0, 1000");

        let mut triangle2 = Polygon::new().set("points", "1000, 1000, 1000, 0, 0, 0");

        // Possibly mirror the triangles
        if random.next_bool() {
            triangle1 = triangle1.set("transform", "scale(-1,1) translate(-1000)");
            triangle2 = triangle2.set("transform", "scale(-1,1) translate(-1000)");
        }

        // Set the colors and return the result
        if random.roll::<u8>(100) < 80 {
            // Pick two solid colors
            let color_mode = match random.roll::<u8>(2) {
                0 => ColorMode::Light,
                1 => ColorMode::Vibrant,
                _ => panic!("Invalid color mode"),
            };

            let random_color1 = HSL::new_random(random, color_mode, 100).as_string();
            let random_color2 = HSL::new_random(random, color_mode, 100).as_string();

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", random_color1);
            triangle2 = triangle2.set("fill", random_color2);

            vec![triangle1.into(), triangle2.into()]
        } else {
            // Randomize the color mode
            let color_mode = match random.roll::<u8>(2) {
                0 => ColorMode::Light,
                1 => ColorMode::Vibrant,
                _ => panic!("Invalid color mode"),
            };

            // Generate two gradient definitions
            let (random_gradient1, gradient_name1) =
                random_gradient_definition(random, Some(45), color_mode, 100);
            let (random_gradient2, gradient_name2) =
                random_gradient_definition(random, Some(45), color_mode, 100);

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", format!("url(#{gradient_name1})"));
            triangle2 = triangle2.set("fill", format!("url(#{gradient_name2})"));

            vec![
                random_gradient1.into(),
                random_gradient2.into(),
                triangle1.into(),
                triangle2.into(),
            ]
        }
    }
}
