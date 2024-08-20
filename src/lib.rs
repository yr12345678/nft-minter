use scrypto::crypto::hash;
use scrypto::prelude::*;
use types::NFTImage;

pub mod hsl;
pub mod layers;
pub mod nft_generator;
pub mod types;
pub mod utils;

#[blueprint]
#[types(NFTImage, Vec<u8>, Hash, NonFungibleLocalId)]
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
            let image_nft_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<NFTImage>(OwnerRole::None)
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
                used_seeds: KeyValueStore::<Vec<u8>, NonFungibleLocalId>::new_with_registered_type(
                ),
                existing_hashes:
                    KeyValueStore::<Hash, NonFungibleLocalId>::new_with_registered_type(),
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
            // Make sure seed length is multiple of 4
            assert!(seed.len() % 4 == 0, "Seed length must be a multiple of 4!");

            // Make sure we can't reuse seeds
            assert!(
                self.used_seeds.get(&seed).is_none(),
                "Seed was already used! Try another one :)"
            );

            // Generate our SVG data
            let nft_image_data = nft_generator::generate_nft_image_data(&seed);
            let url_encoded_nft_image_data = urlencoding::encode(&nft_image_data).into_owned();
            let svg_data_uri = format!("data:image/svg+xml,{url_encoded_nft_image_data}");
            let svg_data_uri_hash = hash(svg_data_uri.clone());

            // Make sure hash does not yet exist
            assert!(
                self.existing_hashes.get(&svg_data_uri_hash).is_none(),
                "This image already exsists!"
            );

            // Mint the NFT
            let nft_id = NonFungibleLocalId::integer(self.next_nft_id);
            let nft_bucket = self.image_nft_manager.mint_non_fungible::<NFTImage>(
                &nft_id,
                NFTImage {
                    key_image_url: Url::of(svg_data_uri.clone()),
                    name: format!("NFT #{}", self.next_nft_id),
                    // Can't guarantee that all characters will be valid UTF-8, so this is basically best-effort and for fun if someone wants to use their own vanity seed
                    seed: String::from_utf8_lossy(&seed).into_owned(),
                    svg_data: hex::encode(nft_image_data),
                },
            );

            // Add the seed and NonFungibleLocalId to the used_seeds KeyValueStore
            self.used_seeds.insert(seed, nft_id.clone());
            self.existing_hashes
                .insert(svg_data_uri_hash, nft_id.clone());

            // Increment our NFT id counter for the next mint
            self.next_nft_id += 1;

            nft_bucket
        }

        /// Checks if a seed was used.
        ///
        /// Returns a tuple with a bool and optionally a NonFungibleLocalId for the NFT that was minted with this seed.
        pub fn seed_used(&self, seed: Vec<u8>) -> (bool, Option<NonFungibleLocalId>) {
            match self.used_seeds.get(&seed) {
                Some(nflid) => (true, Some(nflid.to_owned())),
                None => (false, None),
            }
        }
    }
}
