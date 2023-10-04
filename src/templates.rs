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

#[derive(TemplateOnce)]
#[template(path = "error.html")]
pub struct ErrorHtml {
    pub error: String,
}