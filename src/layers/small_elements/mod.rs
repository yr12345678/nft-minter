use crate::layers::Layer;
use random::Random;
use small_circle::SmallCircle;
use small_heart::SmallHeart;

pub mod small_circle;
pub mod small_heart;

pub fn random_small_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(1);

    match variant {
        0 => Box::new(SmallCircle),
        // 1 => Box::new(SmallHeart),
        _ => panic!("Unknown element variant"),
    }
}
