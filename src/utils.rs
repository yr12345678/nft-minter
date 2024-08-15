use random::Random;
use svg::node::element::{Definitions, LinearGradient, Stop};

#[derive(Debug, Clone, Copy)]
pub struct HSL {
    pub hue: u16,
    pub saturation: u8,
    pub lightness: u8
}

impl HSL {
    fn new(hue: u16, saturation: u8, lightness: u8) -> Self {
        HSL {
            hue,
            saturation,
            lightness
        }
    }

    fn new_random(random: &mut Random) -> Self {
        let hue = random.in_range::<u16>(1, 360);
        let saturation = random.in_range::<u8>(1, 100);
        let lightness = random.in_range::<u8>(1, 100);

        HSL {
            hue,
            saturation,
            lightness
        }                
    }

    // Method to normalize the hue to stay within 0 to 360
    fn normalize_hue(hue: u16) -> u16 {
        if hue >= 360 {
            hue - 360
        } else {
            hue
        }
    }

    // Method to return the triadic colors
    fn triadic_colors(&self) -> (HSL, HSL, HSL) {
        let hue1 = Self::normalize_hue(self.hue + 120);
        let hue2 = Self::normalize_hue(self.hue + 240);

        (
            *self,
            HSL {
                hue: hue1,
                saturation: self.saturation,
                lightness: self.lightness,
            },
            HSL {
                hue: hue2,
                saturation: self.saturation,
                lightness: self.lightness,
            },
        )
    }

    fn as_string(&self) -> String {
        format!("hsl({},{}%,{}%)", self.hue, self.saturation, self.lightness)
    }
}

/// Generates an RGB string using randomness
pub fn random_color(random: &mut Random) -> String {
    let r = random.roll::<u8>(255);
    let g = random.roll::<u8>(255);
    let b = random.roll::<u8>(255);

    println!("rgb({r},{g},{b})");
    format!("rgb({r},{g},{b})")
}

/// Generates an HSL color using randomness
pub fn random_color_hsl(random: &mut Random) -> HSL {
    HSL::new_random(random)
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
