use crate::layers::Layer;
use random::Random;
use half_circle::HalfCircle;
use three_quarter_circle::ThreeQuarterCircle;
use full_circle::FullCircle;
use concentric_circles::ConcentricCircles;
use triangle::BigTriangle;

pub mod half_circle;
pub mod three_quarter_circle;
pub mod full_circle;
pub mod concentric_circles;
pub mod triangle;

pub fn random_big_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(5);
    match variant {
        0 => Box::new(HalfCircle),
        1 => Box::new(ThreeQuarterCircle),
        2 => Box::new(FullCircle),
        3 => Box::new(ConcentricCircles),
        4 => Box::new(BigTriangle),
        _ => panic!("Unknown element variant"),
    }
}
