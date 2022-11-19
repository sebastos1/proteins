use sailfish::TemplateOnce;
use std::collections::HashMap;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub foods: HashMap<String, HashMap<String, String>>,
    pub x: Vec<&'a str>,
    pub y: Vec<String>,
    pub rng: String,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More<'a> {
    pub order: Vec<&'a str>,
    pub product: String,
    pub foods: HashMap<String, HashMap<String, String>>,
}
