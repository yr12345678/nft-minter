
use random::Random;
use crate::{layers::Layer, utils::{random_gradient_definition, ColorMode, HSL}};
use svg::node::element::{path::Data, Path, Element};
pub struct SmallHeart;

impl Layer for SmallHeart {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Randomly pick our scale. Can't work with floats, so scale is a string.
        let valid_scales = ["scale(0.5)", "scale(1)", "scale(2)"];
        let scale = valid_scales
            .get(random.roll::<usize>(3))
            .expect("Did not find a valid scale. This should never happen.");

        // Pick a random color
        let random_color = HSL::new_vibrant_random(random).as_string();

        // Define the heart shape path
        let data = Data::new()
            .move_to((500, 500))
            .elliptical_arc_to((10, 10, 0, 0, 1, 520, 500))
            .elliptical_arc_to((10, 10, 0, 0, 1, 640, 500))
            .quadratic_curve_to((45, 30, 25, 45))
            .quadratic_curve_to((5, 30, 5, 15))
            .close();

        // Add the data to a path and the path to a pattern, and add that pattern to the definitions
        let path = Path::new()
            .set("d", data)
            .set("transform", *scale)
            .set("fill", random_color); // Translucent white

        vec![path.into()]
    }
}