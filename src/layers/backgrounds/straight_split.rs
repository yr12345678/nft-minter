use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct StraightSplitBackground;

impl Layer for StraightSplitBackground {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the two triangles that will make up the diagonal split background
        let mut rectangle1 = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "100%");

        let mut rectangle2 = Rectangle::new()
            .set("x", 500)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "100%");

        // Possibly apply a rotation
        if random.next_bool() {
            rectangle1 = rectangle1.set("transform", "rotate(90, 500, 500)");
            rectangle2 = rectangle2.set("transform", "rotate(90, 500, 500)");
        }

        // Pick either solid or gradient colors
        if random.roll::<u8>(100) < 80 {
            // Solid colors
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Random colors
                let color_mode = match random.roll::<u8>(2) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    _ => panic!("Invalid color mode"),
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill to the triangles
            rectangle1 = rectangle1.set("fill", color1);
            rectangle2 = rectangle2.set("fill", color2);

            vec![rectangle1.into(), rectangle2.into()]
        } else {
            // Gradients
            let ((gradient1, gradient1_name), (gradient2, gradient2_name)) = if base_color.is_some()
            {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, None, color1, color2),
                    gradient_definition(random, None, color3, color4),
                )
            } else {
                // Generate random gradients
                let color_mode = match random.roll::<u8>(2) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    _ => panic!("Invalid color mode"),
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            // Add the fill to the triangles
            rectangle1 = rectangle1.set("fill", format!("url(#{gradient1_name})"));
            rectangle2 = rectangle2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                rectangle1.into(),
                rectangle2.into(),
            ]
        }
    }
}
