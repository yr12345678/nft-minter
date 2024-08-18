use crate::layers::Layer;
pub use hearts::HeartsPattern;
use random::Random;

pub mod hearts;

pub fn random_pattern(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);
    match variant {
        0 => Box::new(HeartsPattern),
        _ => panic!("Unknown element variant"),
    }
}
