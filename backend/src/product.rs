use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Localization {
    x: i64,
    y: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shop {
    id: u64,
    local: Localization,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    ean: u64,
    name: String,
    photo: String,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ProductShop {
    id: u64,
    ean: u64,
    cost: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReturnInfo {
    cost: f64,
    name_product: String,
    distance: f64,
    name_shop: String,
    photo: String,
}

pub fn initialize_products() -> Vec<Product> {
    vec![
        Product {
            ean: 5901534001752,
            name: String::from("Pieczywo żytnie chrupkie"),
            photo: String::from("IMG_20211203_135633.jpg"),
        },
        Product {
            ean: 5900340000560,
            name: String::from("Placek drożdżowy"),
            photo: String::from("IMG_20211203_135659.jpg"),
        },
        Product {
            ean: 5900437081106,
            name: String::from("Babeczki czekoladowe"),
            photo: String::from("IMG_20211203_135734.jpg"),
        },
        Product {
            ean: 5900862217422,
            name: String::from("Galaretka truskawkowy smak"),
            photo: String::from("IMG_20211203_135809.jpg"),
        },
        Product {
            ean: 5905133528740,
            name: String::from("Vita Aloe"),
            photo: String::from("IMG_20211203_140118.jpg"),
        },
        Product {
            ean: 40084176,
            name: String::from("kinder country"),
            photo: String::from("IMG_20211203_140131.jpg"),
        },
        Product {
            ean: 5901125001390,
            name: String::from("Top orzeszki ziemne smażone"),
            photo: String::from("IMG_20211203_140150.jpg"),
        },
        Product {
            ean: 5900571000025,
            name: String::from("felix Orzeszki ziemne"),
            photo: String::from("IMG_20211203_140200.jpg"),
        },
        Product {
            ean: 5900446014607,
            name: String::from("Polaris Juicy"),
            photo: String::from("IMG_20211203_140212.jpg"),
        },
        Product {
            ean: 5900749022392,
            name: String::from("Migdały blanszowane"),
            photo: String::from("IMG_20211203_140235.jpg"),
        },
        Product {
            ean: 5900587044068,
            name: String::from("Chipsy jabłkowe"),
            photo: String::from("IMG_20211203_140244.jpg"),
        },
        Product {
            ean: 5905617003596,
            name: String::from("Orzechy laskowe"),
            photo: String::from("IMG_20211203_140258.jpg"),
        },
        Product {
            ean: 8006540240960,
            name: String::from("Lenor Fresh Air"),
            photo: String::from("IMG_20211203_140317.jpg"),
        },
        Product {
            ean: 5901958612381,
            name: String::from("Morelowo mus jabłkowy"),
            photo: String::from("IMG_20211203_140334.jpg"),
        },
        Product {
            ean: 5449000297044,
            name: String::from("Kinley TonicWater"),
            photo: String::from("IMG_20211203_140356.jpg"),
        },
        Product {
            ean: 5900497341011,
            name: String::from("7up"),
            photo: String::from("IMG_20211203_140411.jpg"),
        },
        Product {
            ean: 5900497301015,
            name: String::from("PepsiCola"),
            photo: String::from("IMG_20211203_140422.jpg"),
        },
        Product {
            ean: 5905187004023,
            name: String::from("Monster Munch"),
            photo: String::from(".jpg"),
        },
        Product {
            ean: 5900699102502,
            name: String::from("Radler warka"),
            photo: String::from(".jpg"),
        },
        Product {
            ean: 5900910008460,
            name: String::from("Mimi Draże"),
            photo: String::from(".jpg"),
        },
        Product {
            ean: 0,
            name: String::from(""),
            photo: String::from("jpg"),
        },
        Product {
            ean: 0xFFFFFFFFFFFFFFFF,
            name: String::from("Hackaton"),
            photo: String::from("IMG_20211203_145458.jpg"),
        },
    ]
}

pub fn initialize_shops() -> Vec<Shop> {
    vec![
        Shop {
            id: 1,
            local: Localization { x: -1, y: -2 },
            name: String::from("Lidl"),
        },
        Shop {
            id: 2,
            local: Localization { x: -10, y: 2 },
            name: String::from("Biedronka"),
        },
        Shop {
            id: 3,
            local: Localization { x: 1, y: -7 },
            name: String::from("Żabka"),
        },
        Shop {
            id: 4,
            local: Localization { x: -5, y: 4 },
            name: String::from("Delikatesy"),
        },
        Shop {
            id: 5,
            local: Localization { x: -14, y: 1 },
            name: String::from("Lidl"),
        },
        Shop {
            id: 6,
            local: Localization { x: 19, y: -18 },
            name: String::from("Biedronka"),
        },
        Shop {
            id: 7,
            local: Localization { x: -15, y: 20 },
            name: String::from("Żabka"),
        },
        Shop {
            id: 8,
            local: Localization { x: -11, y: -5 },
            name: String::from("Delikatesy"),
        },
        Shop {
            id: 9,
            local: Localization { x: -6, y: 9 },
            name: String::from("Lidl"),
        },
        Shop {
            id: 10,
            local: Localization { x: 2, y: 6 },
            name: String::from("Biedronka"),
        },
        Shop {
            id: 11,
            local: Localization { x: 15, y: 10 },
            name: String::from("Żabka"),
        },
        Shop {
            id: 12,
            local: Localization { x: 1, y: -9 },
            name: String::from("Delikatesy"),
        },
        Shop {
            id: 13,
            local: Localization { x: -3, y: 15 },
            name: String::from("Biedronka"),
        },
        Shop {
            id: 14,
            local: Localization { x: -1, y: -19 },
            name: String::from("Żabka"),
        },
        Shop {
            id: 15,
            local: Localization { x: -5, y: -16 },
            name: String::from("Delikatesy"),
        },
    ]
}

pub fn initialize_product_shop() -> Vec<ProductShop> {
    vec![
        ProductShop {
            id: 1,
            ean: 40084176,
            cost: 420.69,
        },
        ProductShop {
            id: 2,
            ean: 5901125001390,
            cost: 0.0,
        },
        ProductShop {
            id: 2,
            ean: 5900437081106,
            cost: 0.0,
        },
        ProductShop {
            id: 2,
            ean: 5900699102502,
            cost: 0.0,
        },
        ProductShop {
            id: 2,
            ean: 5905133528740,
            cost: 0.0,
        },
        ProductShop {
            id: 2,
            ean: 5905187004023,
            cost: 0.0,
        },
        ProductShop {
            id: 2,
            ean: 8006540240960,
            cost: 0.0,
        },
        ProductShop {
            id: 3,
            ean: 5900910008460,
            cost: 0.0,
        },
        ProductShop {
            id: 4,
            ean: 5905187004023,
            cost: 0.0,
        },
        ProductShop {
            id: 5,
            ean: 5900699102502,
            cost: 0.0,
        },
        ProductShop {
            id: 6,
            ean: 5905133528740,
            cost: 0.0,
        },
        ProductShop {
            id: 7,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 8,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 9,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 10,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 11,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 12,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 13,
            ean: 5901534001752,
            cost: 0.0,
        },
        ProductShop {
            id: 14,
            ean: 5900749022392,
            cost: 0.0,
        },
        ProductShop {
            id: 15,
            ean: 5901534001752,
            cost: 0.0,
        },
    ]
}

pub fn get_data(
    product_shop: Vec<ProductShop>,
    products: Vec<Product>,
    shops: Vec<Shop>,
    ean: u64,
) -> Json<Vec<ReturnInfo>> {
    let mut temp_shop: Shop;
    let mut temp_product: Product;
    let mut return_value: Vec<ReturnInfo> = Vec::new();
    let mut found: bool = false;
    for i in product_shop.iter() {
        if ean == i.ean {
            temp_product = find_product(products.clone(), i.ean);
            temp_shop = find_shop(shops.clone(), i.id);
            return_value.push(ReturnInfo {
                cost: i.cost,
                name_product: temp_product.name,
                distance: find_distance(temp_shop.clone(), Localization { x: 0, y: 0 }),
                name_shop: temp_shop.name,
                photo: temp_product.photo,
            });
            found = true;
        }
    }
    if !found {
        temp_product = find_product(products.clone(), products[products.len() - 1].ean);
        return_value.push(ReturnInfo {
            cost: 34.69,
            name_product: temp_product.name,
            distance: 21.37,
            name_shop: String::from("Papieżowy"),
            photo: temp_product.photo,
        });
    }
    Json(return_value)
}

fn get_cheapest_product(product_shop: Vec<ProductShop>) -> ProductShop {
    let mut smallest: f64 = 99999999999.99;
    let mut smallest_index: ProductShop = product_shop[0];
    for i in product_shop.clone() {
        if i.cost < smallest {
            smallest = i.cost;
            smallest_index = i.clone();
        }
    }
    smallest_index
}

fn find_product_shop(products_shops: Vec<ProductShop>, id_ean: u64) -> Vec<ProductShop> {
    let mut return_value: Vec<ProductShop> = Vec::new();
    if id_ean < 15 && id_ean > 0 {
        println!("We are here");
        for i in products_shops.iter() {
            if id_ean == i.id {
                return_value.push(i.clone());
            }
        }
    } else {
        println!("We are there");
        for i in products_shops.iter() {
            if id_ean == i.ean {
                return_value.push(i.clone());
            }
        }
    }
    return_value
}

fn find_closest_shop(shops: Vec<Shop>, local: Localization) -> (Shop, f64) {
    let mut distance: f64 = 999999999999999.99;
    let mut distance_object: Option<Shop> = None;
    let mut current: f64;
    for i in shops.iter() {
        current = ((i.local.x - local.x) * (i.local.x - local.x)
            + (i.local.y - local.y) * (i.local.y - local.y)) as f64;
        current = current.powf(0.5);
        if current < distance {
            distance = current;
            distance_object = Some(i.clone());
        }
    }
    (distance_object.unwrap(), distance)
}

fn find_distance(shop: Shop, local: Localization) -> f64 {
    (((shop.local.x - local.x) * (shop.local.x - local.x)
        + (shop.local.y - local.y) * (shop.local.y - local.y)) as f64)
        .powf(0.5)
}

pub fn find_product(products: Vec<Product>, ean: u64) -> Product {
    for i in products.iter() {
        if i.ean == ean {
            return i.clone();
        }
    }
    products[products.len() - 1].clone()
}

fn find_shop(shops: Vec<Shop>, id: u64) -> Shop {
    for i in shops.iter() {
        if i.id == id {
            return i.clone();
        }
    }
    shops[shops.len() - 1].clone()
}
