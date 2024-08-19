use std::any::{Any, TypeId};

use crate::layers::Layer;
use random::Random;
use scrypto::{info, prelude::ToPrimitive};
use small_circle::SmallCircle;
use stacked_triangles::StackedTriangles;

pub mod small_circle;
pub mod stacked_triangles;

pub fn random_small_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(SmallCircle),
        Box::new(StackedTriangles)
    ];

    let allowed_layers: Vec<Box<dyn Layer>> = available_layers
        .into_iter()
        .filter(|layer| {
            !exclusions.contains(&layer.layer_type())
        })
        .collect();

    if !allowed_layers.is_empty() {
        let variant = random
        .roll::<u8>(allowed_layers.len().to_u8().unwrap())
        .to_usize()
        .unwrap();

        Some(allowed_layers.into_iter().nth(variant).unwrap())
    } else {
        None
    }
}
