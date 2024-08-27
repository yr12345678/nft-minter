use crate::{layers::Layer, utils::pick_random_layer};
use background_checkerboard::BackgroundCheckerboard;
use background_diagonal_split::BackgroundDiagonalSplit;
use background_double_diagonal_split::BackgroundDoubleDiagonalSplit;
use background_four_squares::BackgroundFourSquares;
use random::Random;
use background_rectangle::BackgroundRectangle;
use background_straight_split::BackgroundStraightSplit;
use background_threeway_split::BackgroundThreeWaySplit;
use background_two_stripes::BackgroundTwoStripes;

pub mod background_checkerboard;
pub mod background_diagonal_split;
pub mod background_double_diagonal_split;
pub mod background_four_squares;
pub mod background_rectangle;
pub mod background_straight_split;
pub mod background_threeway_split;
pub mod background_two_stripes;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(BackgroundRectangle), 100),
        (Box::new(BackgroundTwoStripes), 25),
        (Box::new(BackgroundDiagonalSplit), 100),
        (Box::new(BackgroundStraightSplit), 100),
        (Box::new(BackgroundFourSquares), 100),
        (Box::new(BackgroundThreeWaySplit), 100),
        (Box::new(BackgroundDoubleDiagonalSplit), 25),
        (Box::new(BackgroundCheckerboard), 15),
    ];

    // Pick a random layer
    pick_random_layer(random, available_layers)
        .expect("Could not pick a background. This should never happen.")
}
