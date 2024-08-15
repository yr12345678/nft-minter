use crate::layers::Layer;
use random::Random;
use half_circle::HalfCircle;
use three_quarter_circle::ThreeQuarterCircle;

pub mod half_circle;
pub mod three_quarter_circle;

pub fn random_big_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(2);
    match variant {
        0 => Box::new(HalfCircle),
        1 => Box::new(ThreeQuarterCircle),
        _ => panic!("Unknown element variant"),
    }
}
