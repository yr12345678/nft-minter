use std::any::TypeId;

use crate::layers::Layer;
use arch::SmallElementArch;
use flower::SmallElementFlower;
use four_circles::SmallElementFourCircles;
use random::Random;
use scrypto::prelude::ToPrimitive;
use small_circle::SmallElementCircle;
use small_element_square::SmallElementSquare;
use split_circle::SmallElementSplitCircle;
use split_circle_opposite::SmallElementSplitCircleOpposite;
use star::SmallElementStar;

pub mod arch;
pub mod flower;
pub mod four_circles;
pub mod small_circle;
pub mod small_element_square;
pub mod split_circle;
pub mod split_circle_opposite;
pub mod star;

pub fn random_small_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(SmallElementCircle),
        Box::new(SmallElementSquare),
        Box::new(SmallElementArch),
        Box::new(SmallElementSplitCircle),
        Box::new(SmallElementSplitCircleOpposite),
        Box::new(SmallElementFourCircles),
        Box::new(SmallElementFlower),
        Box::new(SmallElementStar),
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
