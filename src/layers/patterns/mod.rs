use crate::layers::Layer;
use random::Random;

pub fn random_pattern(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);
    match variant {
        _ => panic!("Unknown element variant"),
    }
}
