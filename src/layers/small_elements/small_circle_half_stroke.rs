use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Element, Path};

pub struct SmallElementCircleHalfStroke;

impl Layer for SmallElementCircleHalfStroke {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(75, 100) * 2;

        // Create one half of the circle with just the stroke
        let data1 = Data::new()
            .move_to((500 - random_radius, 500))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_radius, 500));

        let mut path1 = Path::new()
            .set("d", data1)
            .set("stroke-width", 100)
            .set("fill", "none");

        // Create the other half with the fill
        let data2 = Data::new()
            .move_to((500 - random_radius - 50, 500))
            .elliptical_arc_to((50, 50, 0, 0, 0, 500 + random_radius + 50, 500));

        let mut path2 = Path::new().set("d", data2);

        // Possibly add a rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        if random.next_bool() {
            path1 = path1.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
            path2 = path2.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
        }

        // Set the fill. Always solid, because gradients don't work well here because of the two parts.
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

        path1 = path1.set("stroke", color.clone());
        path2 = path2.set("fill", color);

        vec![path1.into(), path2.into()]
    }
}
