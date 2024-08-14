use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct ImageNft {
    pub key_image_url: Url,
    pub name: String,
    pub svg_data: String,
}
