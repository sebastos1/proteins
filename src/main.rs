use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let url = std::fs::read_to_string("link.txt").unwrap();

    let res = reqwest::blocking::get(url).unwrap()
        .text().unwrap();
    let map: HashMap<String, Value> = serde_json::from_str(&res).unwrap();

    let mut foods = HashMap::new();
    for x in 0..map["foods"].as_array().unwrap().len() {
        let mut nutrients = HashMap::new();

        macro_rules! insert {
            ($a:expr, $b:expr) => {
                nutrients.insert(
                    $a,
                    map["foods"][x][$b]["value"].as_str().unwrap()
                );
            }
        }

        insert!("kJ", "Energi1");
        insert!("kcal", "Energi2");
        insert!("Protein", "Protein");
        insert!("Karbohydrater", "Karbo");
        insert!("Fett", "Fett");
        insert!("Fiber", "Fiber");

        foods.insert(
            map["foods"][x]["name"].as_str().unwrap(),
            nutrients
        );
    }
    println!("{:#?}", foods);
}