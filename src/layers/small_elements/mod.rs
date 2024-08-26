use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use arch::SmallElementArch;
use cross::SmallElementCross;
use cube::SmallElementCube;
use flower::SmallElementFlower;
use four_circles::SmallElementFourCircles;
use random::Random;
use small_circle::SmallElementCircle;
use small_element_square::SmallElementSquare;
use small_triangle::SmallElementTriangle;
use split_circle::SmallElementSplitCircle;
use split_circle_opposite::SmallElementSplitCircleOpposite;
use star::SmallElementStar;

pub mod arch;
pub mod cross;
pub mod cube;
pub mod flower;
pub mod four_circles;
pub mod small_circle;
pub mod small_element_square;
pub mod small_triangle;
pub mod split_circle;
pub mod split_circle_opposite;
pub mod star;

pub fn random_small_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(SmallElementCircle), 100),
        (Box::new(SmallElementSquare), 100),
        (Box::new(SmallElementArch), 100),
        (Box::new(SmallElementSplitCircle), 100),
        (Box::new(SmallElementSplitCircleOpposite), 100),
        (Box::new(SmallElementFourCircles), 20),
        (Box::new(SmallElementFlower), 100),
        (Box::new(SmallElementStar), 100),
        (Box::new(SmallElementCross), 100),
        (Box::new(SmallElementCube), 100),
        (Box::new(SmallElementTriangle), 100),
    ];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
