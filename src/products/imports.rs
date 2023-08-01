use super::models::{Product};


pub fn read_products(path: String) -> Vec<Product> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut products: Vec<Product> = Vec::new();
    for result in rdr.deserialize() {
        let record: Product = result.unwrap();
        products.push(record);
    }
    products
}
