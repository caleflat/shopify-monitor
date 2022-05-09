use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub variants: Vec<Variant>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub id: i64,
    pub title: String
}

// implement partial_eq for Product
impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}