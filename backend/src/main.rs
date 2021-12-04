#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::{json::Json};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
mod product;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
        .attach(CORS)
        .mount("/", routes![index])
        .mount("/images/", FileServer::from(relative!("imgs")))
        .mount("/", routes![get_item])
        .mount("/", routes![get_item_default])
        .mount("/", routes![get_photos])
}
