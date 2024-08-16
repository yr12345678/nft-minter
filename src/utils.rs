use random::Random;
use svg::node::element::{Definitions, LinearGradient, Stop};

pub enum ColorMode {
    Normal,
    Vibrant,
    Light
}

// TODO: move to separate file
#[derive(Debug, Clone, Copy)]
pub struct HSL {
    pub hue: i16,
    pub saturation: i8,
    pub lightness: i8
}

impl HSL {
    pub fn new(hue: i16, saturation: i8, lightness: i8) -> Self {
        HSL {
            hue,
            saturation,
            lightness
        }
    }

    // Generates a random HSL struct
    pub fn new_random(random: &mut Random) -> Self {
        let hue = random.in_range::<u16>(0, 360);
        let saturation = random.in_range::<u8>(0, 100);
        let lightness = random.in_range::<u8>(0, 100); 

        HSL {
            hue: hue.try_into().unwrap(),
            saturation: saturation.try_into().unwrap(),
            lightness: lightness.try_into().unwrap()
        }                
    }     

    // Generates a vibrant random HSL struct
    pub fn new_vibrant_random(random: &mut Random) -> Self {
        let hue = random.in_range::<u16>(0, 360);
        let saturation = random.in_range::<u8>(80, 100); 
        let lightness = random.in_range::<u8>(40, 60);

        HSL {
            hue: hue.try_into().unwrap(),
            saturation: saturation.try_into().unwrap(),
            lightness: lightness.try_into().unwrap()
        }                
    }  

    // Generates a vibrant random HSL struct
    pub fn new_light_random(random: &mut Random) -> Self {
        let hue = random.in_range::<u16>(0, 360);
        let saturation = 100; 
        let lightness = random.in_range::<u8>(60, 85);

        HSL {
            hue: hue.try_into().unwrap(),
            saturation: saturation.try_into().unwrap(),
            lightness: lightness.try_into().unwrap()
        }                
    }     

    // Method to normalize the hue to stay within 0 to 360
    fn normalize_hue(hue: i16) -> i16 {
        if hue >= 360 {
            hue - 360
        } else {
            hue
        }
    }

    // Method to normalize the hue to stay within 0 to 100
    fn normalize_lightness(lightness: i8) -> i8 {
        if lightness >= 100 {
            lightness - 100
        } else {
            lightness
        }
    }    

    // Method to return the triadic colors
    pub fn triadic_colors(&self) -> (HSL, HSL, HSL) {
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

    // Method to return analogous colors
    pub fn analogous_colors(&self) -> (HSL, HSL, HSL) {
        let (hue1, hue2, hue3) = if self.hue <= 30 {
            // Base color is near 0, so increase hue for the other two variants
            (self.hue, self.hue + 30, self.hue + 60)
        } else if self.hue >= 330 {
            // Base color is near 100, so decrease hue for the other two variants
            (self.hue - 60, self.hue - 30, self.hue)
        } else {
            // Base color is in the middle, adjust both directions
            (self.hue - 30, self.hue, self.hue + 30)
        };        

        (
            HSL {
                hue: hue1,
                saturation: self.saturation,
                lightness: self.lightness
            },
            HSL {
                hue: hue2,
                saturation: self.saturation,
                lightness: self.lightness
            },
            HSL {
                hue: hue3,
                saturation: self.saturation,
                lightness: self.lightness
            },                       
        )
    }

    // Method to return complementary colors
    pub fn complementary_colors(&self) -> (HSL, HSL) {
        let hue1 = Self::normalize_hue(self.hue + 180);

        (
            *self,
            HSL {
                hue: hue1,
                saturation: self.saturation,
                lightness: self.lightness
            }
        )
    }

    // Method to return monochromatic colors
    pub fn monochromatic_colors(&self) -> (HSL, HSL, HSL) {
        let (lightness1, lightness2, lightness3) = if self.lightness <= 10 {
            // Base color is near 0, so increase lightness for the other two variants
            (self.lightness, self.lightness + 10, self.lightness + 20)
        } else if self.lightness >= 90 {
            // Base color is near 100, so decrease lightness for the other two variants
            (self.lightness - 20, self.lightness - 10, self.lightness)
        } else {
            // Base color is in the middle, adjust both directions
            (self.lightness - 10, self.lightness, self.lightness + 10)
        };

        (
            HSL {
                hue: self.hue,
                saturation: self.saturation,
                lightness: lightness1
            },
            HSL {
                hue: self.hue,
                saturation: self.saturation,
                lightness: lightness2
            },
            HSL {
                hue: self.hue,
                saturation: self.saturation,
                lightness: lightness3
            }            
        )
    }

    // Method to return split-complementary colors
    pub fn split_complementary_colors(&self) -> (HSL, HSL, HSL) {
        let hue1 = Self::normalize_hue(self.hue + 150);
        let hue2 = Self::normalize_hue(self.hue - 150);

        (
            *self,
            HSL {
                hue: hue1,
                saturation: self.saturation,
                lightness: self.lightness
            },
            HSL {
                hue: hue2,
                saturation: self.saturation,
                lightness: self.lightness
            }            
        )
    }  

    // Returns triadic colors as strings
    pub fn triadic_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::triadic_colors(self);

        (
            color1.as_string(),
            color2.as_string(),
            color3.as_string()
        )
    }

    // Returns analogous colors as strings
    pub fn analogous_colors_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::analogous_colors(self);

        (
            color1.as_string(),
            color2.as_string(),
            color3.as_string()
        )
    }

    // Returns complementary colors as strings
    // Returns analogous colors as strings
    pub fn complementary_colors_as_string(&self) -> (String, String) {
        let (color1, color2) = Self::complementary_colors(self);

        (
            color1.as_string(),
            color2.as_string()
        )
    }

    // Returns monochromatic colors as strings
    pub fn monochromatic_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::monochromatic_colors(self);

        (
            color1.as_string(),
            color2.as_string(),
            color3.as_string()
        )
    }

    // Returns monochromatic colors as strings
    pub fn split_complementary_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::split_complementary_colors(self);

        (
            color1.as_string(),
            color2.as_string(),
            color3.as_string()
        )
    }       

    // Returns the HSL formatted as a string fit for use in SVG code
    pub fn as_string(&self) -> String {
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
    color_mode: &ColorMode
) -> (Definitions, String) {
    // Get a random base color
    let random_color = match color_mode {
        ColorMode::Normal => HSL::new_random(random),
        ColorMode::Vibrant => HSL::new_vibrant_random(random),
        ColorMode::Light => HSL::new_light_random(random)
    };

    // Generate our color set
    let (random_color1, random_color2) =
    match random.roll::<u8>(4) {
        0 => {
            let (color1, color2, _) = random_color.analogous_colors_colors_as_strings();
            (color1, color2)
        },
        1 => {
            let (color1, color2, _) = random_color.monochromatic_colors_as_strings();
            (color1, color2)
        },
        2 => {
            let (color1, color2, _) = random_color.split_complementary_colors_as_strings();
            (color1, color2)
        },
        3 => {
            let color2 = match color_mode {
                ColorMode::Normal => HSL::new_random(random),
                ColorMode::Vibrant => HSL::new_vibrant_random(random),
                ColorMode::Light => HSL::new_light_random(random)
            };

            (random_color.as_string(), color2.as_string())
        },
        _ => panic!("Invalid color variant")
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
        random_gradient = random_gradient.set("gradientTransform", format!("rotate({rotation})"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(random_gradient);

    (defs, gradient_name)
}
