use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path, Circle};

pub struct SmallElementYinYang;

impl Layer for SmallElementYinYang {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random radius
        let random_radius = random.in_range::<u16>(50, 100) * 2; // Always an even number

        // Generate the circle
        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Generate the swirly thingy
        let data = Data::new()
            .move_to((500, 500 - random_radius))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500, 500))
            .elliptical_arc_to((50, 50, 0, 0, 0, 500, 500 +random_radius))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500, 500 - random_radius));

        let mut path = Path::new().set("d", data);

        // Possible add a rotation
        if random.next_bool() {
            path = path.set("transform", "rotate(90, 500, 500)");
        }

        // Pick random solid colors
        if random.roll::<u8>(100) < 85 {
            // Solid colors
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick random colors
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill
            circle = circle.set("fill", color1);
            path = path.set("fill", color2);

            vec![circle.into(), path.into()]
        } else {
            let ((gradient1, gradient1_name), (gradient2, gradient2_name)) = if base_color.is_some()
            {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                )
            } else {
                // Generate random gradients
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            circle = circle.set("fill", format!("url(#{gradient1_name})"));
            path = path.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                circle.into(),
                path.into(),
            ]
        }
    }
}
