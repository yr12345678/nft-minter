use std::any::Any;

use crate::hsl::*;
use crate::layers::{big_elements, small_elements};
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct ThreeWaySplitBackground;

impl Layer for ThreeWaySplitBackground {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let mut area1 = Polygon::new().set("points", "0,0 500,0 500,500, 0,800");
        let mut area2 = Polygon::new().set("points", "1000,0 500,0 500,500, 1000,800");
        let mut area3 = Polygon::new().set("points", "0,1000 0,800 500,500 1000,800 1000,1000");

        // Possibly rotate the areas
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        if random.next_bool() {
            area1 = area1.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
            area2 = area2.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
            area3 = area3.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
        }

        // Pick either solid or gradient colors
        if random.roll::<u8>(100) < 80 {
            // Solid colors
            let (color1, color2, color3) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Random colors
                let color_mode = match random.roll::<u8>(2) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    _ => panic!("Invalid color mode"),
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill to the triangles
            area1 = area1.set("fill", color1);
            area2 = area2.set("fill", color2);
            area3 = area3.set("fill", color3);

            vec![area1.into(), area2.into(), area3.into()]
        } else {
            // Gradients
            let (
                (gradient1, gradient1_name),
                (gradient2, gradient2_name),
                (gradient3, gradient3_name),
            ) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);
                let color5 = base_color.unwrap().derive_similar_color(random);
                let color6 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                    gradient_definition(random, Some(45), color5, color6),
                )
            } else {
                // Generate random gradients
                let color_mode = match random.roll::<u8>(2) {
                    0 => ColorMode::Light,
                    1 => ColorMode::Vibrant,
                    _ => panic!("Invalid color mode"),
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            // Add the fill to the triangles
            area1 = area1.set("fill", format!("url(#{gradient1_name})"));
            area2 = area2.set("fill", format!("url(#{gradient2_name})"));
            area3 = area3.set("fill", format!("url(#{gradient3_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                gradient3.into(),
                area1.into(),
                area2.into(),
                area3.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::half_circle::HalfCircle.type_id(),
            big_elements::triangle::BigTriangle.type_id(),
            big_elements::two_squares::TwoSquaresElement.type_id(),
            small_elements::stacked_triangles::StackedTriangles.type_id(),
        ]
    }
}
