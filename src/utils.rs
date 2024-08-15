use random::Random;
use svg::node::element::{Definitions, LinearGradient, Stop};

/// Generates an RGB string using randomness
pub fn random_color(random: &mut Random) -> String {
    let r = random.roll::<u8>(255);
    let g = random.roll::<u8>(255);
    let b = random.roll::<u8>(255);

    println!("rgb({r},{g},{b})");
    format!("rgb({r},{g},{b})")
}

/// Generates a gradient using randomness
pub fn random_gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
) -> (Definitions, String) {
    // Generate our colors
    let random_color1 = random_color(random);
    let random_color2 = random_color(random);

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
        random_gradient = random_gradient.set("gradientTransform", format!("rotate({rotation})"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(random_gradient);

    (defs, gradient_name)
}
