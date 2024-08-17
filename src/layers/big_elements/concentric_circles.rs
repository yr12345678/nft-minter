use random::Random;
use crate::{layers::Layer, utils::HSL};
use svg::node::element::{Element, Circle};

pub struct ConcentricCircles;

// TODO: split up gradient and solid into separate variants?
impl Layer for ConcentricCircles {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Randomly select an opacity. Can't use floats, so we use strings here.
        let valid_opacities: [&str; 5] = ["0.3", "0.4", "0.5", "0.6", "0.7"];
        let opacity = valid_opacities
            .get(random.roll::<usize>(5))
            .expect("Did not find a valid opacity. This should never happen.");
    
        // Pick a random color for all circles
        let random_color = if random.roll::<u8>(100) < 50 {
            HSL::new_light_random(random).as_string()
        } else {
            HSL::new_vibrant_random(random).as_string()
        };

        // Generate our 10 circles
        let mut circles: Vec<Element> = Vec::new();
        for i in 1..7 {

            let circle = Circle::new()
                .set("cx", 500)
                .set("cy", 500)
                .set("r", i*80)
                .set("stroke-width", 40)
                .set("fill", "none")
                .set("opacity", *opacity)
                .set("stroke", random_color.clone());

            circles.push(circle.into());
        }

        circles
    }
}