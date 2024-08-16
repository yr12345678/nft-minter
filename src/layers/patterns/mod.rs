use crate::layers::Layer;
use random::Random;
pub use hearts::HeartsPattern;

pub mod hearts;

pub fn random_pattern(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);
    match variant {
        0 => Box::new(HeartsPattern),
        _ => panic!("Unknown element variant"),
    }
}
