use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f32,
}

#[get("/")]
pub fn get_all_products() -> Json<Vec<Product>> {
    let products = vec![
        Product {
            id: 1,
            name: "Product 1".to_string(),
            price: 1.0,
        },
        Product {
            id: 2,
            name: "Product 2".to_string(),
            price: 2.0,
        },
    ];
    Json(products)
}

#[get("/<id>")]
pub fn get_product(id: i32) -> Json<Product> {
    let product = Product {
        id: id,
        name: "Product 1".to_string(),
        price: 99.0,
    };

    Json(product)
}
