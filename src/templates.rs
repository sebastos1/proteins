use sailfish::TemplateOnce;
use std::collections::HashMap;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub ind: usize,
    pub rng: String,
    pub word: String,
    pub showcol: bool,
    pub y: Vec<String>,
    pub entries: usize,
    pub active: Vec<String>,
    pub order: Vec<(&'a str, &'a str)>,
    pub foods: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More<'a> {
    pub product: String,
    pub multiplier: f32,
    pub order: Vec<(&'a str, &'a str)>,
    pub foods: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "paper.html")]
pub struct Paper {
    pub rng: String,
    pub paperitems: Vec<(String, f32)>,
    pub foods: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "custom.html")]
pub struct Custom {}

pub fn active() -> Vec<String> {
    return vec![
        "kcal".to_string(), 
        "Protein".to_string(), 
        "Karbohydrater".to_string(), 
        "Fett".to_string()
    ];
}

pub fn order<'a>() -> Vec<(&'a str, &'a str)> {
    return vec![
        ("kJ", "kJ"),
        ("kcal", "kcal"),
        ("Protein", "g"),
        ("Karbohydrater", "g"),
        ("Fett", "g"),
        ("Hvorav mettet", "g"),
        ("Hvorav enumettet", "g"),
        ("Hvorav flerumettet", "g"),
        ("Tilsatt sukker", "g"),
        ("Vann", "g"),
        ("Salt", "g"),
        ("Fiber", "g"),
        ("Stivelse", "g"),
        ("Kolesterol", "mg"),
        ("Omega-3", "g"),
        ("Transfett", "g"),
        ("Vit A", "µg-RE"),
        ("Vit B1", "mg"),
        ("Vit B2", "mg"),
        ("Vit B6", "mg"),
        ("Vit B12", "µg"),
        ("Vit C", "mg"),
        ("Vit D", "µg"),
        ("Vit E", "mg-ATE"),
        ("Alkohol", "g"),
    ];
}
