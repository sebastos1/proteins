use tokio::fs;
use tokio::fs::File;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use crate::helpers::get_long_names;

pub async fn init() -> HashMap<String, HashMap<String, String>> {
    let mut foods: HashMap<String, HashMap<String, String>>;

    match std::fs::read_to_string("output.json") {
        Ok(file_content) => {
            match serde_json::from_str(&file_content) {
                Ok(data) => {
                    foods = data;
                }
                Err(_) => {
                    println!("[-] Bad data, creating new ...");
                    match update().await {
                        Ok(_) => {
                            foods = try_read_foods("output.json").await;
                        },
                        Err(e) => {
                            println!("[-] Failed to update data, exiting: {}", e);
                            std::process::exit(1);
                        }
                    };
                }
            }
        }
        Err(_) => {
            println!("[-] No data, creating...");
            match update().await {
                Ok(_) => {
                    foods = try_read_foods("output.json").await;
                },
                Err(e) => {
                    println!("[-] Failed to update data, exiting: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }

    if std::path::Path::new("custom.json").exists() {
        match fs::read_to_string("custom.json").await {
            Ok(_) => {
                let custom_foods: HashMap<String, HashMap<String, String>> =
                    try_read_foods("custom.json").await;
                for (k, v) in custom_foods.iter() {
                    foods.insert(k.to_string(), v.clone());
                }
                println!("[+] Custom entries found!")
            },
            Err(_) => {
                println!("[-] Bad custom data: ignoring.");
            }
        }
    }
    foods
}

pub async fn update() -> Result<(), String> {
    let url = match fs::read_to_string("link.txt").await {
        Ok(url) => url,
        Err(e) => {
            println!("[-] Please supply a link to API.");
            return Err(e.to_string());
        }
    };
    let source = "MVT"; // url.clone();

    // infer language from the link ?langauge=no or en
    let language = match url.find("language=") {
        Some(x) => {
            let mut lang = &url[x + 9..x + 11];
            if lang.contains("&") {
                lang = &lang[0..1];
            }
            lang
        }
        None => "en",
    };
    println!("[*] Language: {}", language);

    let res = match fetch_data(&url).await {
        Ok(res) => res,
        Err(e) => {
            println!("[-] Failed to fetch data from '{}'.", url);
            return Err(e.to_string());
        }
    };

    let map: HashMap<String, Value> = match serde_json::from_str(&res) {
        Ok(map) => map,
        Err(e) => {
            println!("[-] Error deserializing JSON");
            return Err(e.to_string());
        }
    };

    let mut foods = HashMap::new();

    for x in 0..map["foods"].as_array().ok_or("Invalid JSON format")?.len() {
        let mut nutrients = HashMap::new();

        macro_rules! insert {
            ($nutrient_name:expr, $nutrient_code:expr) => {
                if let Some(string) = map["foods"][x][$nutrient_code]["value"].as_str() {
                    if string != "0" && !string.is_empty() {
                        nutrients.insert($nutrient_name, string);
                    }
                }
            };
        }

        macro_rules! floor {
            ($nutrient_name:expr, $nutrient_code:expr) => {
                if let Some(mut string) = map["foods"][x][$nutrient_code]["value"].as_str() {
                    if let Some(x) = string.find(".") {
                        string = &string[0..x];
                    };
                    println!("{} {}", $nutrient_name, string);
                    nutrients.insert($nutrient_name, string);
                }
            };
        }

        for (new_name, old_code) in get_long_names(language) {
            if new_name == "kJ" || new_name == "kcal" {
                floor!(new_name, old_code);
            } else {
                println!("{} {}", new_name, old_code);
                insert!(new_name, old_code);
            }
        }

        nutrients.insert("Source", &source);

        let food_name_str = match map["foods"][x]["name"].as_str() {
            Some(name) => name,
            None => {
                println!("[-] Failed to retrieve food name.");
                return Err("Failed to retrieve food name.".to_string());
            }
        };
        
        foods.insert(food_name_str, nutrients);
    }

    match write_foods(&foods).await {
        Ok(_) => {
            println!("[+] Data updated!");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            Err(e.to_string())
        },
    }
}

async fn write_foods(foods: &HashMap<&str, HashMap<&str, &str>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("output.json").await?;
    let json_data = serde_json::to_string(foods)?;
    file.write_all(json_data.as_bytes()).await?;
    Ok(())
}

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn try_read_foods(file: &str) -> HashMap<String, HashMap<String, String>> {
    let foods: HashMap<String, HashMap<String, String>>;

    match std::fs::read_to_string(file) {
        Ok(file_content) => {
            match serde_json::from_str(&file_content) {
                Ok(data) => {
                    foods = data;
                }
                Err(_) => {
                    println!("[-] Failed to parse data for '{}', ignoring.", file);
                    foods = HashMap::new();
                }
            }
        }
        Err(_) => {
            println!("[-] Failed to read data for '{}', ignoring.", file);
            foods = HashMap::new();
        }
    }
    foods
}