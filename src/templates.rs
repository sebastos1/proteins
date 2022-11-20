use sailfish::TemplateOnce;
use std::collections::HashMap;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub foods: HashMap<String, HashMap<String, String>>,
    pub x: Vec<&'a str>,
    pub y: Vec<String>,
    pub rng: String,
    pub sortword: String,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More<'a> {
    pub order: Vec<&'a str>,
    pub product: String,
    pub foods: HashMap<String, HashMap<String, String>>,
}

pub fn order<'a>() -> Vec<&'a str> {
    return vec![
        // "kJ", commented are hard coded in more.html
        // "kcal",
        // "Protein",
        // "Karbohydrater",
        // "Fett",
        "Hvorav mettet",
        "Hvorav enumettet",
        "Hvorav flerumettet",
        "Tilsatt sukker",
        "Vann",
        "Salt",
        "Fiber",
        "Stivelse",
        "Kolesterol",
        "Omega-3",
        "Transfett",
        "Vit A",
        "Vit B1",
        "Vit B2",
        "Vit B6",
        "Vit B12",
        "Vit C",
        "Vit D",
        "Vit E",
        "Alkohol",
    ];
}
