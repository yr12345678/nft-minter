use std::any::TypeId;
use crate::hsl::*;
use random::Random;
use svg::node::element::Element;

pub trait Layer {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element>;
    
    fn exclusions(&self) -> Vec<TypeId> {
        vec![]
    }
}
