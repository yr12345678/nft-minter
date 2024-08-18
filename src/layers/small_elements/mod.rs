use crate::layers::Layer;
use random::Random;
use small_circle::SmallCircle;
use stacked_triangles::StackedTriangles;

pub mod small_circle;
pub mod stacked_triangles;

pub fn random_small_element(random: &mut Random) -> Box<dyn Layer> {
    let variant = random.roll::<u8>(2);

    match variant {
        0 => Box::new(SmallCircle),
        1 => Box::new(StackedTriangles),
        _ => panic!("Unknown element variant"),
    }
}
