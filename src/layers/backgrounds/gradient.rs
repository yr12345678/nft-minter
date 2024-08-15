use crate::layers::Layer;
use crate::utils::random_gradient_definition;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct GradientBackground;

impl Layer for GradientBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate a gradient
        let (random_gradient, gradient_name) = random_gradient_definition(random, None);

        // Generate the rectangle that serves as the background, referring to the gradient we generated
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", format!("url(#{gradient_name})",));

        vec![random_gradient.into(), background.into()]
    }
}
