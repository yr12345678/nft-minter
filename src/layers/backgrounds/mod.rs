use crate::{layers::Layer, utils::pick_random_layer};
use diagonal_split::BackgroundDiagonalSplit;
use double_diagonal_split::BackgroundDoubleDiagonalSplit;
use four_squares::BackgroundFourSquares;
use random::Random;
use rectangle_background::BackgroundRectangle;
use straight_split::BackgroundStraightSplit;
use threeway_split::BackgroundThreeWaySplit;
use two_stripes::BackgroundTwoStripes;

pub mod diagonal_split;
pub mod double_diagonal_split;
pub mod four_squares;
pub mod rectangle_background;
pub mod straight_split;
pub mod threeway_split;
pub mod two_stripes;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(BackgroundRectangle), 100),
        (Box::new(BackgroundTwoStripes), 100),
        (Box::new(BackgroundDiagonalSplit), 100),
        (Box::new(BackgroundStraightSplit), 100),
        (Box::new(BackgroundFourSquares), 100),
        (Box::new(BackgroundThreeWaySplit), 100),
        (Box::new(BackgroundDoubleDiagonalSplit), 25),
    ];

    // Pick a random layer
    pick_random_layer(random, available_layers)
        .expect("Could not pick a background. This should never happen.")
}
