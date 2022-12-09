use serde_json::Value;
use std::collections::HashMap;
use crate::templates::trans;

pub fn init() -> HashMap<String, HashMap<String, String>> {
    let mut foods: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&std::fs::read_to_string("output.json").unwrap()).unwrap();
    if std::path::Path::new("custom.json").exists()
        && &std::fs::read_to_string("custom.json").unwrap() != ""
    {
        let custom_foods: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&std::fs::read_to_string("custom.json").unwrap()).unwrap();
        for (k, v) in custom_foods.iter() {
            foods.insert(k.to_string(), v.clone());
        }
    }
    foods
}

pub fn update() {
    let url = std::fs::read_to_string("link.txt").unwrap();
    let source = url.clone();
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let map: HashMap<String, Value> = serde_json::from_str(&res).unwrap();
    let mut foods = HashMap::new();

    for x in 0..map["foods"].as_array().unwrap().len() {
        let mut nutrients = HashMap::new();

        macro_rules! insert {
            ($a:expr, $b:expr) => {
                let string = map["foods"][x][$b]["value"].as_str().unwrap();
                if string != "0" && !string.is_empty() {
                    Some(nutrients.insert($a, string))
                } else {
                    None
                }
            };
        }

        macro_rules! floor {
            ($a:expr, $b:expr) => {
                let mut string = map["foods"][x][$b]["value"].as_str().unwrap();
                match string.find(".") {
                    Some(x) => string = &string[0..x],
                    None => (),
                };
                nutrients.insert($a, string)
            };
        }

        for (k, v) in trans() {
            if k == "kJ" || k == "kcal" {
                floor!(k, v);
            } else {
                insert!(k, v);
            }
        }

        nutrients.insert("Source", &source);
        
        foods.insert(map["foods"][x]["name"].as_str().unwrap(), nutrients);
    }
    let file = std::fs::File::create("output.json").unwrap();
    serde_json::to_writer(file, &foods).unwrap();
    println!("updated");
}
