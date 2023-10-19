use sailfish::TemplateOnce;
use std::collections::HashMap;
use indexmap::IndexMap;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct Index {
    pub entries_cursor: usize,
    pub rng: String,
    pub currently_sorting_by: String,
    pub show_column_settings: bool,
    pub y: Vec<String>,
    pub entries_per_page: usize,
    pub active_columns: Vec<String>,
    pub dictionary: IndexMap<String, HashMap<String, String>>,
    pub food_data_map: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More {
    pub product: String,
    pub multiplier: f32,
    pub dictionary: IndexMap<String, HashMap<String, String>>,
    pub food_data_map: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "paper.html")]
pub struct Paper {
    pub rng: String,
    pub custom_meal_items: Vec<(String, f32)>,
    pub food_data_map: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "custom.html")]
pub struct Custom {
    pub dictionary: IndexMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "error.html")]
pub struct ErrorHtml {
    pub error: String,
}