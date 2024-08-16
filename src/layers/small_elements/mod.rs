use crate::layers::Layer;
use random::Random;
use small_circle::SmallCircle;

pub mod small_circle;

pub fn random_small_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);

    match variant {
        0 => Box::new(SmallCircle),
        _ => panic!("Unknown element variant"),
    }
}
