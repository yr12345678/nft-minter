use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use random::Random;
use small_element_arch::SmallElementArch;
use small_element_circle::SmallElementCircle;
use small_element_cross::SmallElementCross;
use small_element_cube::SmallElementCube;
use small_element_diagonal_split_square::SmallElementDiagonalSplitSquare;
use small_element_flower::SmallElementFlower;
use small_element_four_circles::SmallElementFourCircles;
use small_element_pill::SmallElementPill;
use small_element_split_circle::SmallElementSplitCircle;
use small_element_split_circle_opposite::SmallElementSplitCircleOpposite;
use small_element_square::SmallElementSquare;
use small_element_stacked_pills::SmallElementStackedPills;
use small_element_star::SmallElementStar;
use small_element_straight_split_square::SmallElementStraightSplitSquare;
use small_element_triangle::SmallElementTriangle;

pub mod small_element_arch;
pub mod small_element_circle;
pub mod small_element_cross;
pub mod small_element_cube;
pub mod small_element_diagonal_split_square;
pub mod small_element_flower;
pub mod small_element_four_circles;
pub mod small_element_pill;
pub mod small_element_split_circle;
pub mod small_element_split_circle_opposite;
pub mod small_element_square;
pub mod small_element_stacked_pills;
pub mod small_element_star;
pub mod small_element_straight_split_square;
pub mod small_element_triangle;

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
        (Box::new(SmallElementPill), 100),
        (Box::new(SmallElementStraightSplitSquare), 100),
        (Box::new(SmallElementStackedPills), 100),
        (Box::new(SmallElementDiagonalSplitSquare), 100),
    ];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
