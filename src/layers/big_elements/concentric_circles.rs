use crate::hsl::*;
use crate::layers::Layer;
use random::Random;
use svg::node::element::{Circle, Element};

pub struct ConcentricCircles;

impl Layer for ConcentricCircles {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly select an opacity. Can't use floats, so we use strings here.
        // let valid_opacities: [&str; 5] = ["0.3", "0.4", "0.5", "0.6", "0.7"];
        // let opacity = valid_opacities
        //     .get(random.roll::<usize>(5))
        //     .expect("Did not find a valid opacity. This should never happen.");

        // Pick a random color for all circles
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

        // Generate our circles
        let mut circles: Vec<Element> = Vec::new();
        for i in 1..5 {
            let circle = Circle::new()
                .set("cx", 500)
                .set("cy", 500)
                .set("r", i * 118)
                .set("stroke-width", 56)
                .set("fill", "none")
                .set("opacity", 1)
                .set("stroke", color.clone());

            circles.push(circle.into());
        }

        circles
    }
}
