use crate::{
    layers::Layer,
    utils::{random_gradient_definition, ColorMode, HSL},
};
use random::Random;
use svg::node::element::{path::Data, Element, Path};

pub struct HalfCircle;

// TODO: split up gradient and solid into separate variants?
impl Layer for HalfCircle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
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
            let random_color = if random.roll::<u8>(100) < 50 {
                HSL::new_light_random(random).as_string()
            } else {
                HSL::new_vibrant_random(random).as_string()
            };
            path = path.set("fill", random_color);

            vec![path.into()]
        } else {
            // Randomize the color mode, but prefer vibrant
            let color_mode = if random.roll::<u8>(100) < 50 {
                ColorMode::Light
            } else {
                ColorMode::Vibrant
            };

            // Get a gradient definition and name and add it as a fill to the path
            let (random_gradient, gradient_name) =
                random_gradient_definition(random, None, &color_mode);
            path = path.set("fill", format!("url(#{gradient_name})",));

            vec![random_gradient.into(), path.into()]
        }
    }
}
