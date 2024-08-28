use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct OverlayTriangle;

impl Layer for OverlayTriangle {
    fn generate(&self, random: &mut Random, _base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let mut triangle = Polygon::new().set("points", "0,0 0,1000 500,500");

        // Add rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        triangle = triangle.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Set the gradient
        let (gradient, gradient_name) = gradient_definition(
            random,
            None,
            HSL { // Opacity = 0
                hue: 0, 
                saturation: 100, 
                lightness: 100, 
                opacity: 0
            },
            HSL { // Opacity = 0
                hue: 0, 
                saturation: 100, 
                lightness: 100, 
                opacity: 100
            }
        );

        triangle = triangle.set("fill", format!("url(#{gradient_name})"));

        // Return the elements
        vec![gradient.into(), triangle.into()]
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}
