use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    id: u32,
    title: String,
    description: String,
    price: f32,
    brand: String,
    category: String,
}
