use std::any::Any;

use crate::layers::Layer;
use crate::utils::*;
use crate::{hsl::*, layers::big_elements};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct FrameStraight;

impl Layer for FrameStraight {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly set stroke width
        let valid_stroke_widths = [100]; // Should be divisable by 2
        let stroke_width = valid_stroke_widths
            .get(random.roll::<usize>(1))
            .expect("Did not find a valid stroke width. This should never happen.");

        // Generate the rectangle that will be our background
        let mut rectangle = Rectangle::new()
            .set("stroke-width", *stroke_width)
            .set("fill", "none")
            .set("x", *stroke_width / 2)
            .set("y", *stroke_width / 2)
            .set("width", 1000 - *stroke_width)
            .set("height", 1000 - *stroke_width);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a darker solid color
            let color = if base_color.is_some() {
                let unwrapped = base_color.unwrap();

                HSL {
                    lightness: unwrapped.lightness - 30,
                    ..unwrapped
                }
                .as_string()
            } else {
                // Pick a random color
                let color_mode = match random.roll::<u8>(3) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    2 => ColorMode::Tone,
                    _ => panic!("Invalid color mode"),
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            rectangle = rectangle.set("stroke", color);

            vec![rectangle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Pick a random color
                let color_mode = match random.roll::<u8>(3) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    2 => ColorMode::Tone,
                    _ => panic!("Invalid color mode"),
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            rectangle = rectangle.set("stroke", format!("url(#{gradient_name})",));

            vec![gradient.into(), rectangle.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        // Prevent the frame from having any big elements on top
        vec![
            big_elements::big_element_square::BigElementSquare.type_id(),
            big_elements::full_circle::BigElementFullCircle.type_id(),
            big_elements::half_circle::BigElementHalfCircle.type_id(),
            big_elements::quarter_circle::BigElementQuarterCircle.type_id(),
            big_elements::three_quarter_circle::BigElementThreeQuarterCircle.type_id(),
            big_elements::triangle::BigElementTriangle.type_id(),
            big_elements::two_squares::BigElementTwoSquares.type_id(),
            big_elements::zig_zag::BigElementZigZag.type_id(),
        ]
    }
}
