use std::any::Any;

use crate::layers::{big_elements, Layer};
use crate::utils::random_gradient_definition;
use crate::{hsl::*, utils::gradient_definition};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct GradientBackground;

impl Layer for GradientBackground {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let (gradient, gradient_name) = if base_color.is_some() {
            // We have a base color, so we derive something similar
            let color1 = base_color.unwrap().derive_similar_color(random);
            let color2 = base_color.unwrap().derive_similar_color(random);

            gradient_definition(random, None, color1, color2)
        } else {
            // Randomize the color mode, but prefer vibrant
            if random.roll::<u8>(100) < 50 {
                random_gradient_definition(random, None, ColorMode::Light, 100)
            } else {
                random_gradient_definition(random, None, ColorMode::Vibrant, 100)
            }
        };

        // Generate the rectangle that serves as the background, referring to the gradient we generated
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", format!("url(#{gradient_name})",));

        vec![gradient.into(), background.into()]
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::two_squares::TwoSquaresElement.type_id() // The two squares big element doesn't differentiate enough from this background
        ]
    }    
}
