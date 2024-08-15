use crate::layers::Layer;
use gradient::GradientBackground;
use random::Random;
use solid::SolidBackground;

pub mod gradient;
pub mod solid;

pub fn random_background(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(2);

    match variant {
        0 => Box::new(SolidBackground),
        1 => Box::new(GradientBackground),
        _ => panic!("Unknown background variant"),
    }
}
