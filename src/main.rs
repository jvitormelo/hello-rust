#[macro_use]
extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/products",
            routes![
                routes::products::get_all_products,
                routes::products::get_product,
            ],
        )
        .mount("/files", routes![routes::file::write_file])
}
