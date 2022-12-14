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
pub struct Custom<'a> {
    pub order: Vec<(&'a str, &'a str)>,
}

pub fn active() -> Vec<String> {
    return vec![
        "kcal".to_string(), 
        "Protein".to_string(), 
        "Karbohydrater".to_string(), 
        "Fett".to_string()
    ];
}

pub fn order<'a>() -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    for (name, unit, _) in big() {
        vec.push((name, unit))
    }
    return vec
}

pub fn trans<'a>() -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    for (new, _, old) in big() {
        vec.push((new, old))
    }
    return vec
}

pub fn big<'a>() -> Vec<(&'a str, &'a str, &'a str)> {
    return vec![
        // new name, unit, original name
        ("kJ", "kJ", "Energi1"),
        ("kcal", "kcal", "Energi2"),
        ("Fett", "g", "Fett"),
        ("Mettet fett", "g", "Mettet"),
        ("Transfett", "g", "Trans"),
        ("Enumettet fett", "g", "Enumet"),
        ("Flerumettet fett", "g", "Flerum"),
        ("Omega-3", "g", "Omega-3"),
        ("Omega-6", "g", "Omega-6"),
        ("Karbohydrater", "g", "Karbo"),
        ("Sukkerarter", "g", "Sukker"),
        ("Stivelse", "g", "Stivel"),
        ("Fiber", "g", "Fiber"),
        ("Protein", "g", "Protein"),
        ("Salt", "g", "NaCl"),
        ("Vann", "g", "Vann"),
        ("Alkohol", "g", "Alko"),
        ("Kolesterol", "mg", "Kolest"),
        ("Vit A", "??g-RE", "Vit A"),
        ("Retinol", "??g", "Retinol"),
        ("Betakaroten", "??g", "B-karo"),
        ("Vit B1", "mg", "Vit B1"),
        ("Vit B2", "mg", "Vit B2"),
        ("Niacin", "mg", "Niacin"),
        ("Vit B6", "mg", "Vit B6"),
        ("Folat", "??g", "Folat"),
        ("Vit B12", "??g", "Vit B12"),
        ("Vit C", "mg", "Vit C"),
        ("Vit D", "??g", "Vit D"),
        ("Vit E", "mg-ATE", "Vit E"),
        ("Kalsium", "mg", "Ca"),
        ("Jern", "mg", "Fe"),
        ("Natrium", "mg", "Na"),
        ("Kalium", "mg", "K"),
        ("Magnesium", "mg", "Mg"),
        ("Sink", "mg", "Zn"),
        ("Selenium", "??g", "Se"),
        ("Kobber", "mg", "Cu"),
        ("Fosfor", "mg", "P"),
        ("Jod", "??g", "I"),
    ];
}
