use crate::layers::Layer;
use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct NFTImage {
    pub key_image_url: Url,
    pub name: String,
    pub seed_lossy: String,
    pub layers: Vec<String>,
    pub svg_data: String,
}

pub struct NFTLayers {
    pub layers: Vec<Box<dyn Layer>>,
}

impl NFTLayers {
    pub fn generate(&self) -> String {
        todo!()
    }
}
