use crate::{layers::Layer, utils::pick_random_layer};
use background_checkerboard::BackgroundCheckerboard;
use background_chevron::BackgroundChevron;
use background_diagonal_split::BackgroundDiagonalSplit;
use background_double_diagonal_split::BackgroundDoubleDiagonalSplit;
use background_four_squares::BackgroundFourSquares;
use background_fourway_split::BackgroundFourWaySplit;
use background_rectangle::BackgroundRectangle;
use background_straight_split::BackgroundStraightSplit;
use background_threeway_split::BackgroundThreeWaySplit;
use background_two_stripes::BackgroundTwoStripes;
use background_zig_zag_split::BackgroundZigZagSplit;
use background_puzzle::BackgroundPuzzle;
use random::Random;

pub mod background_checkerboard;
pub mod background_chevron;
pub mod background_diagonal_split;
pub mod background_double_diagonal_split;
pub mod background_four_squares;
pub mod background_fourway_split;
pub mod background_rectangle;
pub mod background_straight_split;
pub mod background_threeway_split;
pub mod background_two_stripes;
pub mod background_zig_zag_split;
pub mod background_puzzle;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(BackgroundRectangle), 100),
        (Box::new(BackgroundTwoStripes), 10),
        (Box::new(BackgroundDiagonalSplit), 100),
        (Box::new(BackgroundStraightSplit), 100),
        (Box::new(BackgroundFourSquares), 100),
        (Box::new(BackgroundThreeWaySplit), 100),
        (Box::new(BackgroundDoubleDiagonalSplit), 10),
        (Box::new(BackgroundCheckerboard), 10),
        (Box::new(BackgroundZigZagSplit), 100),
        (Box::new(BackgroundChevron), 10),
        (Box::new(BackgroundFourWaySplit), 100),
        (Box::new(BackgroundPuzzle), 100),
    ];

    // Pick a random layer
    pick_random_layer(random, available_layers)
        .expect("Could not pick a background. This should never happen.")
}
