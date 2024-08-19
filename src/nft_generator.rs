use crate::hsl::*;
use crate::layers::*;
use random::Random;
use svg::Document;

pub fn generate_nft_image_data(seed: &Vec<u8>) -> String {
    // Instantiate the randomness
    let mut random = Random::new(seed);

    // Set up our stack of layers
    let mut layers: Vec<Box<dyn Layer>> = Vec::new();

    // Optionally pick a base color
    let base_color = if random.roll::<u8>(100) < 20 {
        Some(HSL::new_random(&mut random, ColorMode::Vibrant, 100))
    } else {
        None
    };

    // Make sure we have at least 2 layers (background + ...)
    while layers.len() < 2 {
        // Start clean
        layers.clear();
        let mut exclusions = vec![];

        // Always add a background
        let background = random_background(&mut random);
        exclusions.append(&mut background.exclusions());
        layers.push(background);

        // Potentially add a pattern
        // if random.roll::<u8>(100) < 10 {
        //     layers.push(random_pattern(&mut random, &base_color));
        // };

        // Potentially add a big element
        if random.next_bool() {
            let big_element = random_big_element(&mut random, &exclusions);
            exclusions.append(&mut big_element.exclusions());
            layers.push(big_element);
        }

        // Potentially add a small element
        if random.next_bool() {
            layers.push(random_small_element(&mut random));
        }
    }

    // Generate the SVG
    let document = generate_svg(layers, &mut random, &base_color);

    document.to_string()
}

fn generate_svg(
    layers: Vec<Box<dyn Layer>>,
    random: &mut Random,
    base_color: &Option<HSL>,
) -> Document {
    // Set up the base Document
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));

    // Iterate through all layers, generate them and add the elements to the Document
    for layer in layers {
        let elements = layer.generate(random, base_color);
        for element in elements {
            document = document.add(element);
        }
    }

    document
}
