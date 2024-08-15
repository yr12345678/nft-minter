use random::Random;
use crate::layers::Layer;
use gradient::GradientBackground;
use solid::SolidBackground;
use three_stripes::ThreeStripesBackground;
use diagonal_split::DiagonalSplitBackground;

pub mod gradient;
pub mod solid;
pub mod three_stripes;
pub mod diagonal_split;

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
