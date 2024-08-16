use crate::layers::*;
use random::Random;
use svg::Document;

pub fn generate_nft_image_data(seed: &Vec<u8>) -> String {
    // Instantiate the randomness
    let mut random = Random::new(seed);

    // Set up our stack of layers
    let mut layers: Vec<Box<dyn Layer>> = Vec::new();

    // Make sure we have at least 2 layers (background + ...)
    while layers.len() < 2 {
        // Start clean
        layers.clear();

        // Always add a background
        layers.push(random_background(&mut random));

        // Potentially add a pattern
        // if random.roll::<u8>(100) < 10 {
        //     layers.push(random_pattern(&mut random));
        // };

        // Potentially add a big element
        if random.next_bool() {
            layers.push(random_big_element(&mut random));
        }

        // Potentially add a small element
        if random.next_bool() {
            layers.push(random_small_element(&mut random));
        }
    }

    // Generate the SVG
    let document = generate_svg(layers, &mut random);

    document.to_string()
}

fn generate_svg(layers: Vec<Box<dyn Layer>>, random: &mut Random) -> Document {
    // Set up the base Document
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));

    // Iterate through all layers, generate them and add the elements to the Document
    for layer in layers {
        let elements = layer.generate(random);
        for element in elements {
            document = document.add(element);
        }
    }

    document
}
