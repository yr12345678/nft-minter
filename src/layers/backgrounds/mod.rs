use crate::layers::Layer;
use diagonal_split::BackgroundDiagonalSplit;
use four_squares::BackgroundFourSquares;
use random::Random;
use rectangle::BackgroundRectangle;
use scrypto::prelude::ToPrimitive;
use straight_split::BackgroundStraightSplit;
use two_stripes::BackgroundTwoStripes;
use threeway_split::BackgroundThreeWaySplit;

pub mod diagonal_split;
pub mod four_squares;
pub mod rectangle;
pub mod straight_split;
pub mod two_stripes;
pub mod threeway_split;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(BackgroundRectangle),
        Box::new(BackgroundTwoStripes),
        Box::new(BackgroundDiagonalSplit),
        Box::new(BackgroundStraightSplit),
        Box::new(BackgroundFourSquares),
        Box::new(BackgroundThreeWaySplit),
    ];

    // Pick a random layer
    let variant = random
        .roll::<u8>(available_layers.len().to_u8().unwrap())
        .to_usize()
        .unwrap();

    available_layers.into_iter().nth(variant).unwrap()
}
