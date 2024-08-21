use std::any::TypeId;

use crate::layers::Layer;
// use concentric_circles::ConcentricCircles;
use full_circle::BigElementFullCircle;
use half_circle::BigElementHalfCircle;
use quarter_circle::BigElementQuarterCircle;
use random::Random;
use scrypto::prelude::ToPrimitive;
use three_quarter_circle::BigElementThreeQuarterCircle;
use triangle::BigElementTriangle;
use two_squares::BigElementTwoSquares;
use zig_zag::BigElementZigZag;
use rectangle::BigElementRectangle;

pub mod concentric_circles;
pub mod full_circle;
pub mod half_circle;
pub mod quarter_circle;
pub mod three_quarter_circle;
pub mod triangle;
pub mod two_squares;
pub mod zig_zag;
pub mod rectangle;

pub fn random_big_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(BigElementHalfCircle),
        Box::new(BigElementThreeQuarterCircle),
        Box::new(BigElementFullCircle),
        // Box::new(ConcentricCircles),
        Box::new(BigElementTriangle),
        Box::new(BigElementTwoSquares),
        Box::new(BigElementQuarterCircle),
        Box::new(BigElementZigZag),
        Box::new(BigElementRectangle),
    ];

    // Filter out the excluded layers
    let allowed_layers: Vec<Box<dyn Layer>> = available_layers
        .into_iter()
        .filter(|layer| !exclusions.contains(&layer.layer_type()))
        .collect();

    if !allowed_layers.is_empty() {
        // Pick a random layer
        let variant = random
            .roll::<u8>(allowed_layers.len().to_u8().unwrap())
            .to_usize()
            .unwrap();

        Some(allowed_layers.into_iter().nth(variant).unwrap())
    } else {
        None
    }
}
