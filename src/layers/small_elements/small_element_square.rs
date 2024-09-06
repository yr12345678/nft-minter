use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use scrypto::prelude::ToPrimitive;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementSquare;

impl Layer for SmallElementSquare {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the required values for building the rectangle. It will vary in size
        // and we have to adjust its position and corner radius with it.
        let random_dimension = random.in_range::<u16>(75, 125) * 2;
        let rx = random_dimension / 5; // This will just get rounded, which is fine
        let position = 500 - (random_dimension / 2);

        // Build the rectangle
        let mut rectangle = Rectangle::new()
            .set("width", random_dimension)
            .set("height", random_dimension)
            .set("x", position)
            .set("y", position);

        // Possibly add a 45 degree rotation
        if random.next_bool() {
            rectangle = rectangle.set("transform", "rotate(45, 500, 500)");
        }

        // Possibly add rounded corners
        if random.next_bool() {
            rectangle = rectangle.set("rx", rx);
        }

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Possibly add a drop-shadow
        if random.roll::<u8>(100) < 15 {
            let drop_shadow_color = HSL::new(0, 0, 0, 100);
            let (drop_shadow, drop_shadow_name) =
                drop_shadow_definition(random, 0, 0, (random_dimension / 5).to_i8().unwrap(), drop_shadow_color, 70);

            rectangle = rectangle.set("filter", format!("url(#{drop_shadow_name})"));
            elements.push(drop_shadow.into());
        }

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            rectangle = rectangle.set("fill", color);

            elements.push(rectangle.into());
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            rectangle = rectangle.set("fill", format!("url(#{gradient_name})",));

            elements.extend(vec![gradient.into(), rectangle.into()]);
        }

        // Return the vector of elements
        elements
    }
}
