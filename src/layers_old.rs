use path::Data;
use random::Random;
use svg::node::element::*;
use svg::*;

pub enum LayerType {
    Circle,
    HalfCircle,
    Rectangle,
    Background { background_type: BackgroundType },
    Arches,
}

pub enum BackgroundType {
    RadixGradient,
    RandomGradient,
    RandomSolid,
    DiagonalSplit,
    PatternStripes,
}

/// TODO: add document instantiation
pub struct Layer;

impl Layer {
    pub fn add(document: Document, layer_type: LayerType, random: &mut Random) -> SVG {
        match layer_type {
            LayerType::Circle => add_circle(random, document),
            LayerType::Background { background_type } => {
                add_background(random, document, background_type)
            }
            LayerType::HalfCircle => add_half_circle(random, document),
            _ => todo!(),
        }
    }
}

/// Generates a random circle layer
fn add_circle(random: &mut Random, document: Document) -> SVG {
    let random_radius = random.in_range::<u16>(50, 150) * 2;
    // let random_x = random.in_range::<u16>(random_radius, 1000 - random_radius);
    // let random_y = random.in_range::<u16>(random_radius, 1000 - random_radius);
    let random_color = generate_random_color(random);

    let circle = Circle::new()
        .set("cx", 500)
        .set("cy", 500)
        .set("r", random_radius)
        .set("fill", random_color);

    document.add(circle)
}

/// Generates a half circle that covers the top or bottom half
fn add_half_circle(random: &mut Random, document: Document) -> SVG {
    let sweep_flag = random.roll::<u8>(2);
    let data = Data::new()
        .move_to((0, 500))
        .elliptical_arc_to((45, 45, 0, 0, sweep_flag, 1000, 500));

    let path = Path::new().set("d", data);

    // Set the fill
    if random.next_bool() {
        // Pick a solid color
        let random_color = generate_random_color(random);
        let path = path.set("fill", random_color);
        document.add(path)
    } else {
        // Get a gradient
        let (random_gradient, gradient_name) = generate_random_linear_gradient(random, None);

        // Add gradient to definitions
        let defs = Definitions::new().add(random_gradient);
        let path = path.set("fill", format!("url(#{gradient_name})",));
        document.add(defs).add(path)
    }
}

///
/// Generates diagonal lines
///

/// Generates a background
///
fn add_background(random: &mut Random, document: Document, background_type: BackgroundType) -> SVG {
    match background_type {
        BackgroundType::RadixGradient => {
            // Set up Radix gradient background
            let radix_gradient_stop1 = Stop::new().set("offset", "0%").set("stop-color", "#1cdcfb");

            let radix_gradient_stop2 = Stop::new()
                .set("offset", "50%")
                .set("stop-color", "#052bc0");

            let radix_gradient_stop3 = Stop::new()
                .set("offset", "100%")
                .set("stop-color", "#fe42cb");

            let radix_gradient = LinearGradient::new()
                .set("id", "radix")
                .set("x1", "0%")
                .set("x2", "100%")
                .set("y1", "0%")
                .set("y2", "0%")
                .add(radix_gradient_stop1)
                .add(radix_gradient_stop2)
                .add(radix_gradient_stop3);

            // Add definitions
            let defs = Definitions::new().add(radix_gradient);
            let document = document.add(defs);

            // Add the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", "url(#radix)");

            document.add(background)
        }
        BackgroundType::RandomGradient => {
            let (random_gradient, gradient_name) = generate_random_linear_gradient(random, None);

            // Add definitions
            let defs = Definitions::new().add(random_gradient);
            let document = document.add(defs);

            // Add the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("url(#{gradient_name})",));

            document.add(background)
        }
        BackgroundType::RandomSolid => {
            let random_color = generate_random_color(random);

            // Add the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", random_color);            

            document.add(background)
        },
        BackgroundType::PatternStripes => {
            let random_color1 = generate_random_color(random);
            let random_color2 = generate_random_color(random);
            let random_color3 = generate_random_color(random);
            let valid_rotate_amounts = [-45, 0, 45, 90];
            let rotate_amount = valid_rotate_amounts
                .get(random.roll::<usize>(4))
                .expect("Did not find a valid rotation amount. This should never happen.");
            let valid_stroke_widths = [10, 20, 40, 50]; // must be divisible by 2, but also 1000 must be divisible by it
            let stroke_width = valid_stroke_widths
                .get(random.roll::<usize>(4))
                .expect("Did not find a valid stroke width. This should never happen.");

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

            let pattern = Pattern::new()
                .set("id", "diagonalstripes")
                .set("patternTransform", format!("rotate({rotate_amount})"))
                .set("patternUnits", "userSpaceOnUse")
                .set("width", stroke_width * 3)
                .set("height", 1)
                .add(line1)
                .add(line2)
                .add(line3);

            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", "url(#diagonalstripes)");

            document.add(pattern).add(background)
        },
        BackgroundType::DiagonalSplit => {
            if random.next_bool() {
                let random_color1 = generate_random_color(random);
                let random_color2 = generate_random_color(random);              

                let mut triangle1 = Polygon::new()
                    .set("points", "0, 0, 1000, 1000, 0, 1000")
                    .set("fill", random_color1);

                let mut triangle2 = Polygon::new()
                    .set("points", "1000, 1000, 1000, 0, 0, 0")
                    .set("fill", random_color2);

                // Possibly mirror
                if random.next_bool() {
                    triangle1 = triangle1.set("transform", "scale(-1,1) translate(-1000)");
                    triangle2 = triangle2.set("transform", "scale(-1,1) translate(-1000)");
                }

                document
                    .add(triangle1)
                    .add(triangle2)
            } else {
                let (random_gradient1, gradient_name1) = generate_random_linear_gradient(random, Some(45));
                let (random_gradient2, gradient_name2) = generate_random_linear_gradient(random, Some(45));

                let defs = Definitions::new()
                    .add(random_gradient1)
                    .add(random_gradient2);

                let document = document.add(defs);

                let mut triangle1 = Polygon::new()
                    .set("points", "0, 0, 1000, 1000, 0, 1000")
                    .set("fill", format!("url(#{gradient_name1})"));

                let mut triangle2 = Polygon::new()
                    .set("points", "1000, 1000, 1000, 0, 0, 0")
                    .set("fill", format!("url(#{gradient_name2})"));

                // Possibly mirror
                if random.next_bool() {
                    triangle1 = triangle1.set("transform", "scale(-1,1) translate(-1000)");
                    triangle2 = triangle2.set("transform", "scale(-1,1) translate(-1000)");
                }                

                document
                    .add(triangle1)
                    .add(triangle2)
            }
        }
    }
}

/// Helper function to quickly generate an RGB string
fn generate_random_color(random: &mut Random) -> String {
    let r = random.roll::<u8>(255);
    let g = random.roll::<u8>(255);
    let b = random.roll::<u8>(255);

    format!("rgb({r},{g},{b})")
}

fn generate_random_linear_gradient(random: &mut Random, rotation: Option<u16>) -> (LinearGradient, String) {
    // Set up random gradient
    let random_color1 = generate_random_color(random);
    let random_color2 = generate_random_color(random);

    let random_gradient_stop1 = Stop::new()
        .set("offset", "0%")
        .set("stop-color", random_color1);

    let random_gradient_stop2 = Stop::new()
        .set("offset", "100%")
        .set("stop-color", random_color2);

    let gradient_name = format!("rg{}", random.in_range::<u16>(0, 65535));
    let mut random_gradient = LinearGradient::new()
        .set("id", gradient_name.clone())
        .set("x1", "0%")
        .set("x2", "100%")
        .set("y1", "0%")
        .set("y2", "0%")
        .add(random_gradient_stop1)
        .add(random_gradient_stop2);

    if rotation.is_some() {
        let rotation = rotation.unwrap();
        random_gradient = random_gradient.set("gradientTransform", format!("rotate({rotation})"));
    }

    (random_gradient, gradient_name)
}
