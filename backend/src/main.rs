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

#[get("/json/photos")]
fn get_photos() -> Json<Vec<product::Product>> {
  let products = product::initialize_products();
  let mut data : Vec<product::Product> = Vec::new();
  data.push(product::find_product(products.clone(), 5900497301015));
  data.push(product::find_product(products.clone(), 5900571000025));
  data.push(product::find_product(products.clone(), 40084176));
  data.push(product::find_product(products.clone(), 5900862217422));
  data.push(product::find_product(products.clone(), 5900437081106));
  data.push(product::find_product(products.clone(), 5901534001752));
  Json (data)
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![index])
        .mount("/images/", FileServer::from(relative!("imgs")))
        .mount("/", routes![get_item])
        .mount("/", routes![get_item_default])
        .mount("/", routes![get_photos])
}
