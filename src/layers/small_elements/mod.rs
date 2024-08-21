use std::any::TypeId;

use crate::layers::Layer;
use random::Random;
use rounded_rectangle::SmallElementRoundedRectangle;
use scrypto::prelude::ToPrimitive;
use small_circle::SmallElementCircle;
// use stacked_triangles::SmallElementStackedTriangles;

pub mod rounded_rectangle;
pub mod small_circle;
// pub mod stacked_triangles;

pub fn random_small_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(SmallElementCircle),
        // Box::new(SmallElementStackedTriangles),
        Box::new(SmallElementRoundedRectangle),
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
