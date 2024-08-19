use std::any::{Any, TypeId};

use crate::layers::Layer;
use concentric_circles::ConcentricCircles;
use full_circle::FullCircle;
use half_circle::HalfCircle;
use random::Random;
use scrypto::prelude::ToPrimitive;
use three_quarter_circle::ThreeQuarterCircle;
use triangle::BigTriangle;

pub mod concentric_circles;
pub mod full_circle;
pub mod half_circle;
pub mod three_quarter_circle;
pub mod triangle;

pub fn random_big_element(random: &mut Random, exclusions: &[TypeId]) -> Box<dyn Layer> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(HalfCircle),
        Box::new(ThreeQuarterCircle),
        Box::new(FullCircle),
        Box::new(ConcentricCircles),
        Box::new(BigTriangle)
    ];

    let allowed_layers: Vec<Box<dyn Layer>> = available_layers
        .into_iter()
        .filter(|layer| {
            !exclusions.contains(&layer.type_id())
        })
        .collect();

    let variant = random
        .roll::<u8>(allowed_layers.len().to_u8().unwrap())
        .to_usize()
        .unwrap();

    allowed_layers.into_iter().nth(variant).unwrap()
}
