use rocket::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Localization {
    x: i64,
    y: i64,
}

#[derive(Serialize, Deserialize)]
struct Shop {
    ID: u64,
    local: Localization,
    name: String,
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
    cost: f64
}

fn initialize_products() -> Vec<Product> {
    vec![{5901534001752, "Pieczywo żytnie chrupkie", "IMG_20211203_135633.jpg"}
    {5900340000560, "Placek drożdżowy","IMG_20211203_135659.jpg"},
    {5900437081106, "Babeczki czekoladowe", "IMG_20211203_135734.jpg"},
    {5900862217422, "Galaretka truskawkowy smak", "IMG_20211203_135809.jpg"},
    {5905133528740, "Vita Aloe", "IMG_20211203_140118.jpg"},
    {40084176, "kinder country", "IMG_20211203_140131.jpg"},
    {5901125001390, "Top orzeszki ziemne smażone", "IMG_20211203_140150.jpg"},
    {5900571000025, "felix Orzeszki ziemne", "IMG_20211203_140200.jpg"},
    {5900446014607, "Polaris Juicy", "IMG_20211203_140212.jpg"},
    {5900749022392, "Migdały blanszowane", "IMG_20211203_140235.jpg"},
    {5900587044068, "Chipsy jabłkowe", "IMG_20211203_140244.jpg"},
    {5905617003596, "Orzechy laskowe", "IMG_20211203_140258.jpg"},
    {8006540240960, "Lenor Fresh Air", "IMG_20211203_140317.jpg"},
    {5901958612381, "Morelowo mus jabłkowy", "IMG_20211203_140334.jpg"},
    {5449000297044, "Kinley TonicWater", "IMG_20211203_140356.jpg"},
    {5900497341011, "7up", "IMG_20211203_140411.jpg"},
    {5900497301015, "PepsiCola", "IMG_20211203_140422.jpg"},
    {5905187004023, "Monster Munch", ".jpg"},
    {5900699102502, "Radler warka", ".jpg"},
    {5900910008460, "Mimi Draże", ".jpg"},
    {, "", ".jpg"},
    {-1, "Hackaton", "IMG_20211203_145458.jpg"}
    ]
}

fn initialize_shops() -> Vec<Shops> {
    vec![{1, {-1, -2},   "Lidl"},
         {2, {-10, 2},   "Biedronka"},
         {3, {1, -7},    "Żabka"},
         {4, {-5, 4},    "Delikatesy"},
         {5, {-14, 1},   "Lidl"},
         {6, {19, -18},  "Biedronka"},
         {7, {-15, 20},  "Żabka"},
         {8, {-11, -5},  "Delikatesy"},
         {9, {-6, 9},    "Lidl"},
         {10, {2, 6},    "Biedronka"},
         {11, {15, 10},  "Żabka"},
         {12, {1, -9},   "Delikatesy"},
         {13, {-3, 15},  "Biedronka"},
         {14, {-1, -19}, "Żabka"},
         {15, {-5, -16}, "Delikatesy"}]
}

fn initialize_product_shop() -> Vec<Product_Shop> {
    vec![{1, 40084176, 420.69},
    {2, 5901125001390},
    {2, 5900437081106},
    {2, 5900699102502},
    {2, 5905133528740},
    {2, 5905187004023},
    {2, 8006540240960},
    {3, 5900910008460},
    {4, 5905187004023},
    {5, 5900699102502},
    {6, 5905133528740},
    {7, 5901534001752},
    {8, 5901534001752},
    {9, 5901534001752},
    {10, 5901534001752},
    {11, 5901534001752},
    {12, 5901534001752},
    {13, 5901534001752},
    {14, 5900749022392},
    {15, 5901534001752}
    ]
}
