#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::{json::Json};
mod product;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json")]
fn dupa() -> Json<Vec<product::ReturnInfo>> {
  let shops = product::initialize_shops();
  let products = product::initialize_products();
  let product_shop = product::initialize_product_shop();
  product::find_product_shop(product_shop, products, shops, 0)
}

#[launch]
fn rocket() -> _ {
    product::initialize_shops();

    rocket::build()
        .mount("/", routes![index])
        .mount("/images/", FileServer::from(relative!("imgs")))
        .mount("/", routes![dupa])
}
