use crate::layers::Layer;
use crate::utils::random_color;
use random::Random;
use svg::node::element::{path::Data, Element, Path, Pattern, Rectangle};

pub struct HeartsPattern;

impl Layer for HeartsPattern {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Randomly pick our scale
        let valid_scales = [("scale(0.5)", 25), ("scale(1)", 50), ("scale(2)", 100)];
        let (scale, width_height) = valid_scales
            .get(random.roll::<usize>(3))
            .expect("Did not find a valid scale. This should never happen.");

        // Define the heart shape path
        let data = Data::new()
            .move_to((5, 15))
            .elliptical_arc_to((10, 10, 0, 0, 1, 25, 15))
            .elliptical_arc_to((10, 10, 0, 0, 1, 45, 15))
            .quadratic_curve_to((45, 30, 25, 45))
            .quadratic_curve_to((5, 30, 5, 15))
            .close();

        // Add the data to a path and the path to a pattern
        let path = Path::new()
            .set("d", data)
            .set("transform", *scale)
            .set("fill", "hsla(0,100%,100%,0.1)"); // Translucent white

        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set("x", 0)
            .set("y", 0)
            .set("width", *width_height)
            .set("height", *width_height)
            .set("patternUnits", "userSpaceOnUse")
            .add(path);

        // Generate the rectangle that will be filled with the pattern
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", format!("url(#{pattern_name})"));

        vec![pattern.into(), background.into()]
    }
}


// <pattern id="heartsPattern" x="0" y="0" width="25" height="25" patternUnits="userSpaceOnUse">
// <path d="M5 15 
//   A 10 10 0 0 1 25 15
//   A 10 10 0 0 1 45 15
//   Q 45 30 25 45
//   Q 5 30 5 15 Z" fill="red" transform="scale(0.5)"/>
// </pattern>