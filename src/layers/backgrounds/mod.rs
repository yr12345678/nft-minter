use crate::layers::Layer;
use diagonal_split::DiagonalSplitBackground;
use gradient::GradientBackground;
use random::Random;
use solid::SolidBackground;
use three_stripes::ThreeStripesBackground;

pub mod diagonal_split;
pub mod gradient;
pub mod solid;
pub mod three_stripes;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(4);

    match variant {
        0 => Box::new(SolidBackground),
        1 => Box::new(GradientBackground),
        2 => Box::new(ThreeStripesBackground),
        3 => Box::new(DiagonalSplitBackground),
        _ => panic!("Unknown background variant"),
    }
}
