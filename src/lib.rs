use random::Random;
use scrypto::prelude::*;
use types::ImageNft;
use scrypto::crypto::hash;

// SVG related stuff
use svg::node::element::path::Data;
use svg::node::element::{Circle, Definitions, LinearGradient, Path, Rectangle, Stop};
use svg::Document;

pub mod types;

#[blueprint]
mod nft_minter {

    struct NftMinter {
        image_nft_manager: ResourceManager,
        next_nft_id: u64,
        used_seeds: KeyValueStore<Vec<u8>, NonFungibleLocalId>,
        existing_hashes: KeyValueStore<Hash, NonFungibleLocalId>
    }

    impl NftMinter {
        pub fn instantiate() -> Global<NftMinter> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(NftMinter::blueprint_id());

            // Create the NFT manager
            let image_nft_manager = ResourceBuilder::new_integer_non_fungible::<ImageNft>(OwnerRole::None)
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(deny_all);
                    burner_updater => rule!(deny_all);
                ))
                .metadata(metadata! {
                    init {
                        "name" => "NFT Collection", locked;
                        "description" => "An NFT collection", locked;
                        "icon_url" => Url::of("https://commons.wikimedia.org/wiki/File:Bitterballen_mosterd_mayo.jpg"), locked;
                        "tags" => vec!["nft", "collection"], updatable;
                    }
                })                
                .create_with_no_initial_supply();

            // Instantiate
            Self {
                image_nft_manager,
                next_nft_id: 1,
                used_seeds: KeyValueStore::<Vec<u8>, NonFungibleLocalId>::new(),
                existing_hashes: KeyValueStore::<Hash, NonFungibleLocalId>::new()
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .metadata(metadata! (
                init{
                    "name" => "NFT minter", locked;
                    "description" => "NFT minter", locked;
                }
            ))
            .with_address(address_reservation)
            .globalize()
        }

        pub fn mint_nft(&mut self, seed: Vec<u8>) -> Bucket {
            // Make sure we can't reuse seeds
            assert!(
                self.used_seeds.get(&seed).is_none(),
                "Seed was already used!"
            );

            // Generate our SVG data
            let svg_document =
                NftMinter::generate_image_svg_data(&seed);

            let encoded_document = urlencoding::encode(&svg_document).into_owned();
            let svg_data = format!("data:image/svg+xml,{encoded_document}");
            let svg_hash = hash(svg_data.clone());

            // Make sure hash does not yet exist
            assert!(
                self.existing_hashes.get(&svg_hash).is_none(),
                "This image already exsists!"
            );

            // Mint the NFT
            let nft_id = NonFungibleLocalId::integer(self.next_nft_id);
            let nft_bucket = self.image_nft_manager.mint_non_fungible::<ImageNft>(
                &nft_id,
                ImageNft {
                    key_image_url: Url::of(svg_data.clone()),
                    name: format!("NFT #{}", self.next_nft_id),
                    svg_data: svg_document
                },
            );

            // Add the seed and NonFungibleLocalId to the used_seeds KeyValueStore
            self.used_seeds.insert(seed, nft_id.clone());
            self.existing_hashes.insert(svg_hash, nft_id.clone());

            // Increment our NFT id counter for the next mint
            self.next_nft_id += 1;

            nft_bucket
        }

        fn generate_image_svg_data(seed: &Vec<u8>) -> String {
            // Get random number in range and colors
            let mut random = Random::new(seed);
            let eye_random_r = random.in_range::<u8>(0, 255);
            let eye_random_g = random.in_range::<u8>(0, 255);
            let eye_random_b = random.in_range::<u8>(0, 255);
            let face_color = format!("rgb({eye_random_r}, {eye_random_g}, {eye_random_b})");
            let background_1_random_r = random.in_range::<u8>(0, 255);
            let background_1_random_g = random.in_range::<u8>(0, 255);
            let background_1_random_b = random.in_range::<u8>(0, 255);
            let gradient_color_1 = format!(
                "rgb({background_1_random_r}, {background_1_random_g}, {background_1_random_b})"
            );
            let background_2_random_r = random.in_range::<u8>(0, 255);
            let background_2_random_g = random.in_range::<u8>(0, 255);
            let background_2_random_b = random.in_range::<u8>(0, 255);
            let gradient_color_2 = format!(
                "rgb({background_2_random_r}, {background_2_random_g}, {background_2_random_b})"
            );

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

            // Set up random gradient
            let random_gradient_stop1 = Stop::new()
                .set("offset", "0%")
                .set("stop-color", gradient_color_1.clone());

            let random_gradient_stop2 = Stop::new()
                .set("offset", "100%")
                .set("stop-color", gradient_color_2.clone());

            let random_gradient = LinearGradient::new()
                .set("id", "random_gradient")
                .set("x1", "0%")
                .set("x2", "100%")
                .set("y1", "0%")
                .set("y2", "0%")
                .add(random_gradient_stop1)
                .add(random_gradient_stop2);

            let defs = Definitions::new().add(radix_gradient).add(random_gradient);

            // Our eyes
            let eye1 = Circle::new()
                .set("cx", 350)
                .set("cy", 350)
                .set("r", 50)
                .set("fill", face_color.clone());

            let eye2 = Circle::new()
                .set("cx", 650)
                .set("cy", 350)
                .set("r", 50)
                .set("fill", face_color.clone());

            // Mouth
            let mouth_data = Data::new()
                .move_to((250, 650))
                .line_to((750, 650))
                .vertical_line_to(700)
                .line_to((250, 700));

            let mouth = Path::new()
                .set("fill", face_color.clone())
                .set("d", mouth_data);

            // The background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", "url(#random_gradient)");

            // Bring it all together
            Document::new()
                .set("viewBox", (0, 0, 1000, 1000))
                .add(defs)
                .add(background)
                .add(eye1)
                .add(eye2)
                .add(mouth)
                .to_string()
        }
    }
}
