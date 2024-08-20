use std::any::TypeId;

use crate::layers::Layer;
use concentric_circles::ConcentricCircles;
use full_circle::FullCircle;
use half_circle::HalfCircle;
use random::Random;
use scrypto::prelude::ToPrimitive;
use three_quarter_circle::ThreeQuarterCircle;
use triangle::BigTriangle;
use two_squares::TwoSquaresElement;

pub mod concentric_circles;
pub mod full_circle;
pub mod half_circle;
pub mod three_quarter_circle;
pub mod triangle;
pub mod two_squares;

pub fn random_big_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(HalfCircle),
        Box::new(ThreeQuarterCircle),
        Box::new(FullCircle),
        // Box::new(ConcentricCircles),
        Box::new(BigTriangle),
        Box::new(TwoSquaresElement),
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
