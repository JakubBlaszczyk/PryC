use rocket::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Localization {
    x: u64,
    y: u64,
}

#[derive(Serialize, Deserialize)]
struct Shop {
    ID: u64,
    local: Localization,
}

#[derive(Serialize, Deserialize)]
struct Product {
    EAN: u64,
    name: String,
    photo: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Product_Shop {
    ID: u64,
    EAN: u64,
}

#[macro_export]
macro_rules! import_image {
  ($, *) => vec!(include_bytes! ())
}

fn initialize_products() -> Vec<Product> {
  vec![{5901534001752, "Pieczywo żytnie chrupkie", vec!(include_bytes! ("../../imgs/IMG_20211203_135633.jpg"))}
       {5900340000560, "Placek drożdżowy","IMG_20211203_135659"},
       {5900437081106, "Babeczki czekoladowe", "IMG_20211203_135734"},
       {5900862217422, "Galaretka truskawkowy smak", "IMG_20211203_135809"},
       {5905133528740, "Vita Aloe", "IMG_20211203_140118"},
       {40084176, "kinder country", "IMG_20211203_140131"},
       {5901125001390, "Top orzeszki ziemne smażone", "IMG_20211203_140150"},
       {5900571000025, "felix Orzeszki ziemne", "IMG_20211203_140200"},
       {5900446014607, "Polaris Juicy", "IMG_20211203_140212"},
       {, "", "IMG_20211203_140235"},
       {, "", "IMG_20211203_140244"},
       {, "", "IMG_20211203_140258"},
       {, "", "IMG_20211203_140317"},
       {, "", "IMG_20211203_140334"},
       {, "", "IMG_20211203_140356"},
       {, "", "IMG_20211203_140411"},
       {, "", "IMG_20211203_140422"},
       {-1, "Hackaton", "IMG_20211203_145458"}
       ];
}