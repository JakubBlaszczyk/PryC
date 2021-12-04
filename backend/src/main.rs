#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::{json::Json};
mod product;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json/<ean>")]
fn get_item(ean: u64) -> Json<Vec<product::ReturnInfo>> {
  let shops = product::initialize_shops();
  let products = product::initialize_products();
  let product_shop = product::initialize_product_shop();
  product::get_data(product_shop, products, shops, ean)
}

#[get("/json")]
fn get_item_default() -> Json<Vec<product::ReturnInfo>> {
  let shops = product::initialize_shops();
  let products = product::initialize_products();
  let product_shop = product::initialize_product_shop();
  product::get_data(product_shop, products, shops, 0)
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![index])
        .mount("/images/", FileServer::from(relative!("imgs")))
        .mount("/", routes![get_item])
        .mount("/", routes![get_item_default])
}
