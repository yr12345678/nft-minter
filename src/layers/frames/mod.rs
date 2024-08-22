use std::any::TypeId;

use crate::layers::Layer;
use frame_straight::FrameStraight;
use random::Random;
use scrypto::prelude::ToPrimitive;

pub mod frame_straight;

pub fn random_frame(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![Box::new(FrameStraight)];

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
