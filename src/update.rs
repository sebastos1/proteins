use tokio::fs;
use tokio::fs::File;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use crate::helpers::{convert_id, RDI_PATH, FOOD_PATH, CUSTOM_PATH, API_PATH};


pub async fn load_data() -> Result<HashMap<String, HashMap<String, String>>, String> {
    let file_content = std::fs::read_to_string(FOOD_PATH).map_err(|_| "[-] Bad data, creating new ...".to_string())?;
    let mut food_data: HashMap<String, HashMap<String, String>> = serde_json::from_str(&file_content).map_err(|_| "[-] Bad data, creating new ...".to_string())?;

    if std::path::Path::new(CUSTOM_PATH).exists() {
        match fs::read_to_string(CUSTOM_PATH).await {
            Ok(_) => {
                let custom_foods: HashMap<String, HashMap<String, String>> = try_read_foods(CUSTOM_PATH).await?;
                for (k, v) in custom_foods {
                    food_data.insert(k, v);
                }
                println!("[+] Custom entries found!");
            },
            Err(_) => {
                println!("[-] Bad custom data, ignoring.");
            }
        }
    }
    Ok(food_data)
}

async fn try_read_foods(file: &str) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let file_content = std::fs::read_to_string(file).map_err(|_| format!("[-] Failed to read data for '{file}'."))?;
    let foods: HashMap<String, HashMap<String, String>> = serde_json::from_str(&file_content).map_err(|_| format!("[-] Failed to parse data for '{file}'."))?;
    Ok(foods)
}

pub async fn update_food_data() -> Result<HashMap<String, HashMap<String, String>>, String> {
    let api_link = set_api_key_language(read_api_key(API_PATH)?, true);
    let response = fetch_data(&api_link).await.map_err(|_| "[-] Failed to fetch data from '{api_link}'.".to_string())?;
    let api_link = set_api_key_language(read_api_key(API_PATH)?, false);
    let response_english = fetch_data(&api_link).await.map_err(|_| "[-] Failed to fetch data from '{api_link}'.".to_string())?;

    let mut json_map: HashMap<String, Value> = serde_json::from_str(&response).map_err(|_| "[-] Error deserializing JSON".to_string())?;
    let json_map_english: HashMap<String, Value> = serde_json::from_str(&response_english).map_err(|_| "[-] Error deserializing JSON".to_string())?;

    let english_names_lookup: HashMap<String, String> = json_map_english["foods"].as_array().unwrap().iter().map(|food| {
        let id = food["id"].as_str().unwrap().to_string();
        let name = food["name"].as_str().unwrap().to_string();
        (id, name)
    }).collect();

    if let Some(Value::Array(foods)) = json_map.get_mut("foods") {
        for food in foods {
            let id = food["id"].as_str().unwrap();
            if let Some(english_name) = english_names_lookup.get(id) {
                food.as_object_mut().unwrap().insert("english_name".to_string(), Value::String(english_name.clone()));
            }
        }
    }
    
    let mut food_data = HashMap::new();
    let foods = json_map["foods"].as_array().ok_or("Invalid JSON format")?;

    for food in foods {
        let mut nutrients = HashMap::new();

        for (old_code, new_code) in convert_id() {
            if new_code == "kJ" || new_code == "kcal" {
                floor_insert(&mut nutrients, new_code, food, old_code);
            } else {
                regular_insert(&mut nutrients, new_code, food, old_code);
            }
        }

        let food_name = food["name"].as_str().ok_or("Failed to retrieve food name.")?.to_string();
        if let Some(english_name) = english_names_lookup.get(&food["id"].as_str().unwrap().to_string()) {
            nutrients.insert("en".to_string(), english_name.clone());
        }
        
        food_data.insert(food_name, nutrients);
    }

    write_foods(&food_data).await.map_err(|e| format!("[-] Error writing to file: {e}"))?;
    println!("[+] Data updated!");

    Ok(food_data)
}


async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn regular_insert(nutrients: &mut HashMap<String, String>, nutrient_name: &str, food: &Value, nutrient_code: &str) {
    if let Some(string) = food[nutrient_code]["value"].as_str() {
        if string != "0" && !string.is_empty() {
            nutrients.insert(nutrient_name.to_string(), string.to_string());
        }
    }
}

fn floor_insert(nutrients: &mut HashMap<String, String>, nutrient_name: &str, food: &Value, nutrient_code: &str) {
    if let Some(mut string) = food[nutrient_code]["value"].as_str() {
        if let Some(x) = string.find(".") {
            string = &string[0..x];
        };
        nutrients.insert(nutrient_name.to_string(), string.to_string());
    }
}

async fn write_foods(food_data: &HashMap<String, HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(FOOD_PATH).await?;
    let serialized_data = serde_json::to_string(food_data)?;
    file.write_all(serialized_data.as_bytes()).await?;
    Ok(())
}

fn set_api_key_language(api_key: String, norwegian: bool) -> String {
    let lang = if norwegian { "no" } else { "en" };
    
    if api_key.contains("language=") {
        let re = regex::Regex::new(r"language=[^&]*").unwrap();
        re.replace(&api_key, &format!("language={}", lang)).into_owned()
    } else {
        if api_key.contains("?") {
            format!("{}&language={}", api_key, lang)
        } else {
            format!("{}?language={}", api_key, lang)
        }
    }
}

fn read_api_key(link_path: &str) -> Result<String, String> {
    let file_content = std::fs::read_to_string(link_path).map_err(|_| "[-] Failed to read API key.".to_string())?;
    Ok(file_content)
}

pub async fn update_rdis(rdis: HashMap<String, HashMap<String, String>>) {
    let serialized = serde_json::to_string(&rdis).expect("Failed to serialize");
    let mut file = File::create(RDI_PATH).await.unwrap();
    file.write_all(serialized.as_bytes()).await.unwrap();
    println!("[+] RDI data updated!");
}