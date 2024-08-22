use crate::hsl::*;
use crate::layers::*;
use random::Random;
use svg::Document;

pub fn generate_nft_image_data(seed: &Vec<u8>) -> (String, Vec<String>) {
    // Instantiate the randomness
    let mut random = Random::new(seed);

    // Set up our stack of layers
    let mut layers: Vec<Box<dyn Layer>> = Vec::new();

    // Optionally pick a base color
    let base_color = if random.roll::<u8>(100) < 30 {
        Some(HSL::new_random(&mut random, ColorMode::Vibrant, 100))
    } else {
        None
    };

    // Make sure we have at least 2 layers (background + ...)
    while layers.len() < 2 {
        // Start clean
        layers.clear();
        let mut exclusions = vec![];

        // Always add a background and add exclusions to the exclusions list
        let background = random_background(&mut random);
        exclusions.append(&mut background.exclusions());
        layers.push(background);

        // Potentially add a frame and add any exclusions to the exclusions list
        if random.roll::<u8>(100) < 20 && base_color.is_some() {
            let frame = random_frame(&mut random, &exclusions);
            if frame.is_some() {
                let unwrapped = frame.unwrap();
                exclusions.append(&mut unwrapped.exclusions());
                layers.push(unwrapped);
            }
        }         

        // Potentially add a pattern
        // if random.roll::<u8>(100) < 10 {
        //     layers.push(random_pattern(&mut random, &base_color));
        // };

        // Potentially add a big element and add any exclusions to the exclusions list
        if random.next_bool() {
            let big_element = random_big_element(&mut random, &exclusions);
            if big_element.is_some() {
                let unwrapped = big_element.unwrap();
                exclusions.append(&mut unwrapped.exclusions());
                layers.push(unwrapped);
            }
        }

        // Potentially add a small element and add any exclusions to the exclusions list
        if random.next_bool() {
            let small_element = random_small_element(&mut random, &exclusions);
            if small_element.is_some() {
                let unwrapped = small_element.unwrap();
                exclusions.append(&mut unwrapped.exclusions());
                layers.push(unwrapped);
            }
        }       
    }

    // Generate the SVG
    let (document, layer_names) = generate_svg(layers, &mut random, &base_color);

    (document.to_string(), layer_names)
}

fn generate_svg(
    layers: Vec<Box<dyn Layer>>,
    random: &mut Random,
    base_color: &Option<HSL>,
) -> (Document, Vec<String>) {
    // Set up the base Document
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));

    // Vector of layer names
    let mut layer_names: Vec<String> = vec![];

    // Iterate through all layers, generate them and add the elements to the Document
    for layer in layers {
        let elements = layer.generate(random, base_color);
        for element in elements {
            document = document.add(element);
        }

        layer_names.push(layer.layer_name());
    }

    (document, layer_names)
}
