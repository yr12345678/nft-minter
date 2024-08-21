use crate::hsl::*;
use crate::layers::Layer;
use random::Random;
use svg::node::element::{Definitions, Element, Pattern, Polygon, Rectangle};

pub struct BigElementZigZag;

impl Layer for BigElementZigZag {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick the fill, which can be either solid or gradient, with a higher chance of solid than gradient
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

        // Randomly pick a rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the polygon that we will use in the pattern
        let polygon = Polygon::new()
            .set("points", "0,1000 0,850, 100,750 200,850 200,1000")
            .set("fill", color);

        // Generate the pattern
        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set("patternTransform", format!("rotate({rotate_amount})"))
            .set("patternUnits", "userSpaceOnUse")
            .set("width", "200")
            .set("height", "100%")
            .add(polygon);

        let defs = Definitions::new().add(pattern);

        // Generate the rectangle that we fill with the pattern
        let rectangle = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("x", 0)
            .set("y", 0)
            .set("fill", format!("url(#{pattern_name})"));

        vec![defs.into(), rectangle.into()]
    }
}
