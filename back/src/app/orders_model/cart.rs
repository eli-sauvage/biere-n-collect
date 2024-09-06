use crate::app::product_variations::Variation;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
struct CartElement {
    variation: Variation,
    quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cart {
    elements: Vec<CartElement>,
}
