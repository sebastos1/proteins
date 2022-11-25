use serde_json::Value;
use std::collections::HashMap;

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

        floor!("kJ", "Energi1");
        floor!("kcal", "Energi2");
        insert!("Protein", "Protein");
        insert!("Karbohydrater", "Karbo");
        insert!("Fett", "Fett");
        insert!("Fiber", "Fiber");
        insert!("Omega-3", "Omega-3");
        insert!("Hvorav mettet", "Mettet");
        insert!("Hvorav enumettet", "Enumet");
        insert!("Hvorav flerumettet", "Flerum");
        insert!("Tilsatt sukker", "Sukker");
        insert!("Transfett", "Trans");
        insert!("Kolesterol", "Kolest");
        insert!("Stivelse", "Stivel");
        insert!("Salt", "NaCl");
        insert!("Alkohol", "Alko");
        insert!("Vit A", "Vit A");
        insert!("Vit B1", "Vit B1");
        insert!("Vit B2", "Vit B2");
        insert!("Vit B6", "Vit B6");
        insert!("Vit B12", "Vit B12");
        insert!("Vit C", "Vit C");
        insert!("Vit D", "Vit D");
        insert!("Vit E", "Vit E");
        insert!("Vann", "Vann");
        nutrients.insert("Source", &source);
        
        foods.insert(map["foods"][x]["name"].as_str().unwrap(), nutrients);
    }
    let file = std::fs::File::create("output.json").unwrap();
    serde_json::to_writer(file, &foods).unwrap();
}
