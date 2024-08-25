use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use big_element_square::BigElementSquare;
use full_circle::BigElementFullCircle;
use half_circle::BigElementHalfCircle;
use quarter_circle::BigElementQuarterCircle;
use random::Random;
use three_quarter_circle::BigElementThreeQuarterCircle;
use triangle::BigElementTriangle;
use two_squares::BigElementTwoSquares;
use zig_zag::BigElementZigZag;
use pill::BigElementPill;
use pill_split_circle::BigElementPillSplitCircle;

pub mod big_element_square;
pub mod full_circle;
pub mod half_circle;
pub mod quarter_circle;
pub mod three_quarter_circle;
pub mod triangle;
pub mod two_squares;
pub mod zig_zag;
pub mod pill;
pub mod pill_split_circle;

pub fn random_big_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(BigElementHalfCircle), 100),
        (Box::new(BigElementThreeQuarterCircle), 100),
        (Box::new(BigElementFullCircle), 100),
        (Box::new(BigElementTriangle), 100),
        (Box::new(BigElementTwoSquares), 100),
        (Box::new(BigElementQuarterCircle), 100),
        (Box::new(BigElementZigZag), 100),
        (Box::new(BigElementSquare), 100),
        (Box::new(BigElementPill), 100),
        (Box::new(BigElementPillSplitCircle), 100),
    ];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
