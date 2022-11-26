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
    pub ind: usize,
    pub entries: usize,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More<'a> {
    pub order: Vec<(&'a str, &'a str)>,
    pub product: String,
    pub foods: HashMap<String, HashMap<String, String>>,
    pub multiplier: f32,
}

#[derive(TemplateOnce)]
#[template(path = "paper.html")]
pub struct Paper {
    pub foods: HashMap<String, HashMap<String, String>>,
    pub paperitems: Vec<(String, f32)>,
}

#[derive(TemplateOnce)]
#[template(path = "custom.html")]
pub struct Custom {}

pub fn order<'a>() -> Vec<(&'a str, &'a str)> {
    return vec![
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
