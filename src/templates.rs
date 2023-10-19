use sailfish::TemplateOnce;
use std::collections::HashMap;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub entries_cursor: usize,
    pub rng: String,
    pub currently_sorting_by: String,
    pub show_column_settings: bool,
    pub y: Vec<String>,
    pub entries_per_page: usize,
    pub active_columns: Vec<String>,
    pub nutrient_order: Vec<(&'a str, &'a str)>,
    pub food_data_map: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
pub struct More<'a> {
    pub product: String,
    pub multiplier: f32,
    pub nutrient_order: Vec<(&'a str, &'a str)>,
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
pub struct Custom<'a> {
    pub nutrient_order: Vec<(&'a str, &'a str)>,
}

#[derive(TemplateOnce)]
#[template(path = "error.html")]
pub struct ErrorHtml {
    pub error: String,
}