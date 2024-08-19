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
    let random_color = HSL::new_random(random, color_mode, opacity);

    // Generate our color set
    let (random_color1, random_color2) = match random.roll::<u8>(4) {
        0 => {
            let (color1, color2, _) = random_color.analogous_colors();
            (color1, color2)
        }
        1 => {
            let (color1, color2, _) = random_color.monochromatic_colors();
            (color1, color2)
        }
        2 => {
            let (color1, color2, _) = random_color.split_complementary_colors();
            (color1, color2)
        }
        3 => {
            let color2 = HSL::new_random(random, color_mode, opacity);
            (random_color, color2)
        }
        _ => panic!("Invalid color variant"),
    };

    // Generate the gradient with the random colors and return the result
    gradient_definition(random, rotation, random_color1, random_color2)
}

/// Generates a gradient using color input
pub fn gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
    color1: HSL,
    color2: HSL,
) -> (Definitions, String) {
    // Set up the gradient
    let gradient_name = format!("gr{}", random.in_range::<u16>(0, 65535));
    let mut gradient = LinearGradient::new()
        .set("id", gradient_name.clone())
        .add(
            Stop::new()
                .set("offset", "0%")
                .set("stop-color", color1.as_string()),
        )
        .add(
            Stop::new()
                .set("offset", "100%")
                .set("stop-color", color2.as_string()),
        );

    // Apply rotation if necessary
    if rotation.is_some() {
        let rotation = rotation.unwrap();
        gradient = gradient.set("gradientTransform", format!("rotate({rotation}, 0.5, 0.5)"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(gradient);

    (defs, gradient_name)
}
