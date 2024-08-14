use layers::*;
use random::Random;
use scrypto::crypto::hash;
use scrypto::prelude::*;
use types::ImageNft;

// SVG related stuff
use svg::Document;

pub mod layers;
pub mod types;

#[blueprint]
mod nft_minter {

    struct NftMinter {
        image_nft_manager: ResourceManager,
        next_nft_id: u64,
        used_seeds: KeyValueStore<Vec<u8>, NonFungibleLocalId>,
        existing_hashes: KeyValueStore<Hash, NonFungibleLocalId>,
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
                existing_hashes: KeyValueStore::<Hash, NonFungibleLocalId>::new(),
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
            let svg_document = NftMinter::generate_image_svg_data(&seed);

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
                    svg_data: hex::encode(svg_document)
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
            // Instantiate the random
            let mut random = Random::new(seed);

            // Create a new document
            let document = Document::new().set("viewBox", (0, 0, 1000, 1000));

            // Add a background
            let random_number = random.roll::<u8>(100);
            let background_type = if random_number == 0 {
                BackgroundType::RadixGradient
            } else if random_number < 55 {
                BackgroundType::RandomSolid
            } else if random_number < 75 {
                BackgroundType::RandomGradient
            } else if random_number < 85 {
                BackgroundType::DiagonalSplit
            } else {
                BackgroundType::PatternStripes
            };

            let document = layers::Layer::add(
                document,
                LayerType::Background { background_type },
                &mut random,
            );

            // Add a half circle? Chance of adding it is 2:1
            let document = if random.roll::<u8>(3) < 2 {
                layers::Layer::add(document, LayerType::HalfCircle, &mut random)
            } else {
                document
            };

            // Add a whole circle? Chance of adding it is 2:1
            let document = if random.roll::<u8>(3) < 2 {
                layers::Layer::add(document, LayerType::Circle, &mut random)
            } else {
                document
            };

            // Bring it all together
            document.to_string()
        }
    }
}
