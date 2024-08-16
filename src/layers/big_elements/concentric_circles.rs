use random::Random;
use scrypto::prelude::hash_map_new;
use crate::{layers::Layer, utils::{random_color, random_gradient_definition, ColorMode, HSL}};
use svg::node::element::{Element, Circle};

pub struct ConcentricCircles;

// TODO: split up gradient and solid into separate variants?
impl Layer for ConcentricCircles {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        let start_radius = random.in_range::<u16>(25, 40) * 2; // Always an even number
    
        // Pick a random color for all circles
        let random_color = HSL::new_vibrant_random(random).as_string();

        // Generate our 10 circles
        let mut circles: Vec<Element> = Vec::new();
        for i in 1..11 {

            let circle = Circle::new()
                .set("cx", 500)
                .set("cy", 500)
                .set("r", start_radius + i*40)
                .set("stroke-width", 20)
                .set("fill", "none")
                .set("opacity", "0.7")
                .set("stroke", random_color.clone());

            circles.push(circle.into());
        }

        circles
    }
}