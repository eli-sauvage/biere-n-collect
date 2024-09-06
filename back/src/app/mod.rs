pub(crate) mod mail;
pub(crate) mod orders;
pub(crate) mod receipt;
pub(crate) mod stock;
pub(crate) mod stripe;

mod products_model;
pub(crate) use products_model::product_variations;
pub(crate) use products_model::products;

mod orders_model;
pub(crate) use orders_model::orders;
pub(crate) use orders_model::receipt;
