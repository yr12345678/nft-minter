use crate::layers::Layer;
use diagonal_split::DiagonalSplitBackground;
use gradient::GradientBackground;
use random::Random;
use scrypto::prelude::ToPrimitive;
use solid::SolidBackground;
use three_stripes::ThreeStripesBackground;
use straight_split::StraightSplitBackground;
use four_squares::FourSquaresBackground;

pub mod diagonal_split;
pub mod gradient;
pub mod solid;
pub mod three_stripes;
pub mod straight_split;
pub mod four_squares;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(SolidBackground),
        Box::new(GradientBackground),
        Box::new(ThreeStripesBackground),
        Box::new(DiagonalSplitBackground),
        Box::new(StraightSplitBackground),
        Box::new(FourSquaresBackground)
    ];

    // Pick a random layer
    let variant = random
        .roll::<u8>(available_layers.len().to_u8().unwrap())
        .to_usize()
        .unwrap();

    available_layers.into_iter().nth(variant).unwrap()
}
