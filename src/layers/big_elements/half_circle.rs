use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{path::Data, Element, Path};

pub struct BigElementHalfCircle;

impl Layer for BigElementHalfCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick a direction
        let data = match random.roll::<u8>(8) {
            0 => {
                // Inside to top
                Data::new()
                    .move_to((0, 0))
                    .elliptical_arc_to((45, 45, 0, 0, 0, 1000, 0))
            }
            1 => {
                // Inside to right
                Data::new()
                    .move_to((1000, 0))
                    .elliptical_arc_to((45, 45, 0, 0, 0, 1000, 1000))
            }
            2 => {
                // Inside to bottom
                Data::new()
                    .move_to((0, 1000))
                    .elliptical_arc_to((45, 45, 0, 0, 1, 1000, 1000))
            }
            3 => {
                // Inside to left
                Data::new()
                    .move_to((0, 0))
                    .elliptical_arc_to((45, 45, 0, 0, 1, 0, 1000))
            }
            4 => {
                // Outside to top
                Data::new()
                    .move_to((0, 500))
                    .elliptical_arc_to((45, 45, 0, 0, 1, 1000, 500))
            }
            5 => {
                // Outside to right
                Data::new()
                    .move_to((500, 0))
                    .elliptical_arc_to((45, 45, 0, 0, 1, 500, 1000))
            }
            6 => {
                // Outside to bottom
                Data::new()
                    .move_to((0, 500))
                    .elliptical_arc_to((45, 45, 0, 0, 0, 1000, 500))
            }
            7 => {
                // Outside to left
                Data::new()
                    .move_to((500, 0))
                    .elliptical_arc_to((45, 45, 0, 0, 0, 500, 1000))
            }
            _ => panic!("Unknown direction"),
        };

        let mut path = Path::new().set("d", data);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // We have a base color, so we derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let color_mode = match random.roll::<u8>(3) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    2 => ColorMode::Tone,
                    _ => panic!("Invalid color mode"),
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            path = path.set("fill", color);

            vec![path.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, None, color1, color2)
            } else {
                // Pick a random color
                let color_mode = match random.roll::<u8>(3) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    2 => ColorMode::Tone,
                    _ => panic!("Invalid color mode"),
                };

                random_gradient_definition(random, None, color_mode, 100)
            };

            path = path.set("fill", format!("url(#{gradient_name})",));

            vec![gradient.into(), path.into()]
        }
    }
}
