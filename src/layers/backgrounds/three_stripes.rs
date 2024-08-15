use crate::{layers::Layer, utils::HSL};
// use crate::utils::random_color;
use random::Random;
use svg::node::element::{Element, Line, Pattern, Rectangle};

pub struct ThreeStripesBackground;

impl Layer for ThreeStripesBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate random colors for the three stripes
        let (random_color1, random_color2, random_color3) =
            match random.roll::<u8>(4) {
                0 => HSL::new_vibrant_random(random).triadic_colors_as_strings(),
                1 => HSL::new_vibrant_random(random).analogous_colors_colors_as_strings(),
                2 => HSL::new_vibrant_random(random).monochromatic_colors_as_strings(),
                3 => HSL::new_vibrant_random(random).split_complementary_colors_as_strings(),
                _ => panic!("Invalid color variant")
            };

        // Randomly set rotation and stroke widths
        let valid_rotate_amounts = [-45, 0, 45, 90];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        let valid_stroke_widths = [20, 40, 50]; // must be divisible by 2, but also 1000 must be divisible by it
        let stroke_width = valid_stroke_widths
            .get(random.roll::<usize>(3))
            .expect("Did not find a valid stroke width. This should never happen.");

        // Generate the stripes
        let line1 = Line::new()
            .set("x1", stroke_width / 2)
            .set("x2", stroke_width / 2)
            .set("y2", 1)
            .set(
                "style",
                format!("stroke:{random_color1}; stroke-width:{stroke_width}"),
            );

        let line2 = Line::new()
            .set("x1", (stroke_width / 2) + stroke_width)
            .set("x2", (stroke_width / 2) + stroke_width)
            .set("y2", 1)
            .set(
                "style",
                format!("stroke:{random_color2}; stroke-width:{stroke_width}"),
            );

        let line3 = Line::new()
            .set("x1", (stroke_width / 2) + stroke_width * 2)
            .set("x2", (stroke_width / 2) + stroke_width * 2)
            .set("y2", 1)
            .set(
                "style",
                format!("stroke:{random_color3}; stroke-width:{stroke_width}"),
            );

        // Add the stripes to a pattern
        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set("patternTransform", format!("rotate({rotate_amount})"))
            .set("patternUnits", "userSpaceOnUse")
            .set("width", stroke_width * 3)
            .set("height", 1)
            .add(line1)
            .add(line2)
            .add(line3);

        // Create a rectangle with that pattern, which serves as the background
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", format!("url(#{pattern_name})"));

        vec![pattern.into(), background.into()]
    }
}
