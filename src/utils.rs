use crate::hsl::*;
use random::Random;
use svg::node::element::{Definitions, LinearGradient, Stop};

/// Generates a gradient using randomness
pub fn random_gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
    color_mode: ColorMode,
    opacity: i8,
) -> (Definitions, String) {
    // Get a random base color
    let random_color = HSL::new_random(random, color_mode.clone(), opacity);

    // Generate our color set
    let (random_color1, random_color2) = match random.roll::<u8>(4) {
        0 => {
            let (color1, color2, _) = random_color.analogous_colors_as_strings();
            (color1, color2)
        }
        1 => {
            let (color1, color2, _) = random_color.monochromatic_colors_as_strings();
            (color1, color2)
        }
        2 => {
            let (color1, color2, _) = random_color.split_complementary_colors_as_strings();
            (color1, color2)
        }
        3 => {
            let color2 = HSL::new_random(random, color_mode, opacity);
            (random_color.as_string(), color2.as_string())
        }
        _ => panic!("Invalid color variant"),
    };

    // Set up the gradient
    let gradient_name = format!("rg{}", random.in_range::<u16>(0, 65535));
    let mut random_gradient = LinearGradient::new()
        .set("id", gradient_name.clone())
        .add(
            Stop::new()
                .set("offset", "0%")
                .set("stop-color", random_color1),
        )
        .add(
            Stop::new()
                .set("offset", "100%")
                .set("stop-color", random_color2),
        );

    // Apply rotation if necessary
    if rotation.is_some() {
        let rotation = rotation.unwrap();
        random_gradient =
            random_gradient.set("gradientTransform", format!("rotate({rotation}, 0.5, 0.5)"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(random_gradient);

    (defs, gradient_name)
}
