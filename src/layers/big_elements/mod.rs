use crate::layers::Layer;
use random::Random;
use half_circle::HalfCircle;

pub mod half_circle;

pub fn random_big_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);

    match variant {
        0 => Box::new(HalfCircle),
        _ => panic!("Unknown element variant"),
    }
}
