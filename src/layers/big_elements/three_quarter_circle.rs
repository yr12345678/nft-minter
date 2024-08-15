use random::Random;
use crate::{layers::Layer, utils::{random_color, random_gradient_definition, ColorMode, HSL}};
use svg::node::element::{path::Data, Element, Path};

pub struct ThreeQuarterCircle;

// TODO: split up gradient and solid into separate variants?
impl Layer for ThreeQuarterCircle {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Randomly pick a direction
        let data = match random.roll::<u8>(4) {
            0 => { // Bottom-left quarter cut
                Data::new()
                    .move_to((0, 500))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 500, 1000))
                    .line_to((500, 500))
                    .close()
            },
            1 => { // Top-left quarter cut
                Data::new()
                    .move_to((500, 0))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 0, 500))
                    .line_to((500, 500))
                    .close()
            },
            2 => { // Top-right quarter cut
                Data::new()
                    .move_to((1000, 500))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 500, 0))
                    .line_to((500, 500))
                    .close()
            },
            3 => { // Bottom-right quarter cut
                Data::new()
                    .move_to((500, 1000))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 1000, 500))
                    .line_to((500, 500))
                    .close()
            },
            _ => panic!("Invalid circle variant")                
        };

        let mut path = Path::new().set("d", data);

        // Set the fill, which can be either solid or gradient with a 50/50 chance
        if random.next_bool() {
            // Pick a solid color
            let random_color = HSL::new_random(random).as_string();
            path = path.set("fill", random_color);

            vec![path.into()]
        } else {
            // Randomize the color mode, but prefer vibrant
            let color_mode = if random.roll::<u8>(100) < 10 {
                ColorMode::Normal
            } else {
                ColorMode::Vibrant
            };

            // Get a gradient definition and name and add it as a fill to the path
            let (random_gradient, gradient_name) = random_gradient_definition(random, None, &color_mode);
            path = path.set("fill", format!("url(#{gradient_name})",));

            vec![random_gradient.into(), path.into()]
        }
    }
}