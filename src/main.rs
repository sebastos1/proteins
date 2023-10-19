use http::Uri;
use rand::Rng;
use warp::Filter;
use std::sync::{Arc, Mutex};
use warp::reply::html;
use urlencoding::decode;
use sailfish::TemplateOnce;
use std::collections::HashMap;
mod update;
mod helpers;
mod templates;
use crate::update::*;
use crate::helpers::*;
use crate::templates::*;

#[tokio::main]
async fn main() {
    // food dictionary: id { unit, parent, order_pos, en, no }
    let dictionary = load_dictionary();
    // println!("{:#?}", dictionary);

    // food data
    let food_data_map = init().await;
    let mut food_names: Vec<String> = food_data_map.clone().into_keys().collect();
    food_names.sort_by_key(|name| name.to_lowercase());
    let foods_currently_shown = Arc::new(Mutex::new(<Vec<String>>::new()));
    
    // ordering
    let currently_sorting_by = Arc::new(Mutex::new(String::new()));

    // scrolling
    let entries_cursor = Arc::new(Mutex::new(0));
    let entries_per_page = 10;

    // columns
    let active_columns = Arc::new(Mutex::new(get_active_columns()));
    let show_column_settings = Arc::new(Mutex::new(false));

    let custom_meal_items = Arc::new(Mutex::new(<Vec<(String, f32)>>::new()));
    let unique_id = Arc::new(Mutex::new(rand::thread_rng().gen::<u32>()));

    let index = warp::path!().map({
        let unique_id = unique_id.clone();
        let entries_cursor = entries_cursor.clone();
        let food_names = food_names.clone();
        let currently_sorting_by = currently_sorting_by.clone();
        let dictionary = dictionary.clone();
        let foods_currently_shown = foods_currently_shown.clone();
        let food_data_map = food_data_map.clone();
        let active_columns = active_columns.clone();
        let entries_per_page = entries_per_page.clone();
        let show_column_settings = show_column_settings.clone();
        move || {
            *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
            let foods_currently_shown = foods_currently_shown.lock().unwrap();

            let y: Vec<String> = if !foods_currently_shown.is_empty() {
                foods_currently_shown.to_vec()
            } else {
                food_names.clone()
            };

            html(
                Index {
                    y,
                    food_data_map: food_data_map.clone(),
                    dictionary: dictionary.clone(),
                    entries_per_page: entries_per_page.clone(),
                    entries_cursor: *entries_cursor.lock().unwrap(),
                    show_column_settings: *show_column_settings.lock().unwrap(),
                    rng: unique_id.lock().unwrap().to_string(),
                    currently_sorting_by: currently_sorting_by.lock().unwrap().to_string(),
                    active_columns: active_columns.lock().unwrap().to_vec(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let entries_cursor = entries_cursor.clone();
            let food_names = food_names.clone();
            let foods_currently_shown = foods_currently_shown.clone();
            move |form: Vec<(String, String)>| {
                *entries_cursor.lock().unwrap() = 0;

                let mut valid: Vec<String> = Vec::new();
                let search = form[0].1.split(" ").collect::<Vec<&str>>();
                for item in &food_names {
                    if search
                        .iter()
                        .all(|term| item.to_lowercase().contains(&term.to_lowercase()))
                        && !valid.contains(item)
                    {
                        valid.push(item.to_string())
                    }
                }
                *foods_currently_shown.lock().unwrap() = valid;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let entries_cursor = entries_cursor.clone();
            let food_names = food_names.clone();
            let currently_sorting_by = currently_sorting_by.clone();
            let food_data_map = food_data_map.clone();
            let foods_currently_shown = foods_currently_shown.clone();
            move |form: Vec<(String, String)>| {
                *entries_cursor.lock().unwrap() = 0;
                let mut foods_currently_shown = foods_currently_shown.lock().unwrap();
                let mut currently_sorting_by = currently_sorting_by.lock().unwrap();

                let sort: Vec<String>;
                let mut sorted: Vec<String> = Vec::new();
                if foods_currently_shown.is_empty() {
                    sort = food_names.clone();
                } else {
                    sort = (*foods_currently_shown.clone()).to_vec();
                }

                let mut collected: Vec<(String, u32)> = Vec::new();
                for prod in &sort {
                    if food_data_map[prod].contains_key(&form[0].1) && food_data_map[prod][&form[0].1] != "" {
                        collected.push((
                            prod.clone(),
                            (food_data_map[prod][&form[0].1].trim().parse::<f32>().unwrap() * 100.) as u32,
                        ));
                    } else {
                        collected.push((prod.clone(), 0. as u32))
                    }
                }

                if form[0].1 != currently_sorting_by.to_string() {
                    collected.sort_by(|a, b| b.1.cmp(&a.1));
                    *currently_sorting_by = form[0].1.clone();
                } else {
                    collected.sort_by(|a, b| a.1.cmp(&b.1));
                    *currently_sorting_by = format!("{} (desc.)", form[0].1.clone());
                }

                for item in collected {
                    sorted.push(item.0)
                }
                *foods_currently_shown = sorted.clone();

                warp::redirect(Uri::from_static("/"))
            }
        });

    let show_column_settings = warp::path!("columns").map({
        let show_column_settings = show_column_settings.clone();
        move || {
            let lol = *show_column_settings.lock().unwrap();
            if lol == false {
                *show_column_settings.lock().unwrap() = true
            } else {
                *show_column_settings.lock().unwrap() = false
            }
            warp::redirect(Uri::from_static("/"))
        }
    });
    let column = warp::path!("column")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let active_columns = active_columns.clone();
            move |form: Vec<(String, String)>| {
                let mut new = Vec::<String>::new();
                for (item, _) in &form[1..] {
                    new.push(item.to_string());
                }
                *active_columns.lock().unwrap() = new;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let change = warp::path!("change")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let entries_cursor = entries_cursor.clone();
            move |form: Vec<(String, String)>| {
                let val = *entries_cursor.lock().unwrap();
                if form[0].1 == "up" {
                    *entries_cursor.lock().unwrap() = val + entries_per_page;
                } else if val >= entries_per_page {
                    *entries_cursor.lock().unwrap() = val - entries_per_page;
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    let product = warp::path("product").and(warp::path::param()).map({
        let dictionary = dictionary.clone();
        let food_data_map = food_data_map.clone();
        move |product: String| {
            html(
                More {
                    multiplier: 1.0,
                    dictionary: dictionary.clone(),
                    food_data_map: food_data_map.clone(),
                    product: urlencoding::decode(&product).unwrap().to_string(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let amount = warp::path!("amount")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let unique_id = unique_id.clone();
            let dictionary = dictionary.clone();
            let food_data_map = food_data_map.clone();
            move |form: Vec<(String, String)>| {
                *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                html(
                    More {
                        dictionary: dictionary.clone(),
                        food_data_map: food_data_map.clone(),
                        product: form[0].1.to_string(),
                        multiplier: form[1].1.parse::<f32>().unwrap() / 100.,
                    }
                    .render_once()
                    .unwrap(),
                )
            }
        });

    let paper = warp::path("paper").map({
        let unique_id = unique_id.clone();
        let food_data_map = food_data_map.clone();
        let custom_meal_items = custom_meal_items.clone();
        move || {
            html(
                Paper {
                    food_data_map: food_data_map.clone(),
                    rng: unique_id.lock().unwrap().to_string(),
                    custom_meal_items: custom_meal_items.lock().unwrap().to_vec(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let add = warp::path!("add")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let unique_id = unique_id.clone();
            let custom_meal_items = custom_meal_items.clone();
            move |form: Vec<(String, String)>| {
                *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                let multiplier = form[1].1.parse::<f32>().unwrap();
                let mut custom_meal_items = custom_meal_items.lock().unwrap();

                if custom_meal_items.is_empty() {
                    custom_meal_items.push((form[0].1.clone(), multiplier));
                } else {
                    let mut contains = false;
                    for (i, (prod, mul)) in custom_meal_items.clone().iter().enumerate() {
                        if prod == &form[0].1 {
                            custom_meal_items[i].1 = mul + multiplier;
                            contains = true;
                        }
                    }
                    if contains == false {
                        custom_meal_items.push((form[0].1.clone(), multiplier));
                    }
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    let remove = warp::path!("remove")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let unique_id = unique_id.clone();
            let custom_meal_items = custom_meal_items.clone();
            move |form: Vec<(String, String)>| {
                *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                let mut custom_meal_items = custom_meal_items.lock().unwrap();
                custom_meal_items.retain(|item| item.0 != form[0].1);
                warp::redirect(Uri::from_static("/paper"))
            }
        });

    let clear = warp::path!("clear").map({
        let custom_meal_items = custom_meal_items.clone();
        move || {
            *custom_meal_items.lock().unwrap() = <Vec<(String, f32)>>::new();
            warp::redirect(Uri::from_static("/paper"))
        }
    });

    // submitting doesnt work yet
    let custom = warp::path("custom").map({
        let dictionary = dictionary.clone();
        move || {
            html(
                Custom {
                    dictionary: dictionary.clone(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let insert = warp::path!("insert")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            move |form: Vec<(String, String)>| {
                let mut nutrients = HashMap::new();
                for (k, v) in &form[1..] {
                    nutrients.insert(k.to_string(), format!("{:.1}", v.parse::<f32>().unwrap()));
                }
                nutrients.insert("Source".to_string(), "custom".to_string());
                nutrients.insert(
                    "kJ".to_string(),
                    format!("{:.1}", form[1].1.parse::<f32>().unwrap() * 4.2),
                );

                let mut food: HashMap<String, HashMap<String, String>>;
                if std::path::Path::new("data/custom.json").exists()
                    && std::fs::read_to_string("data/custom.json").unwrap() != ""
                {
                    food = serde_json::from_str(&std::fs::read_to_string("data/custom.json").unwrap())
                        .unwrap();
                } else {
                    food = HashMap::new();
                }
                food.insert(form[0].1.to_string(), nutrients);
                let file = std::fs::File::create("data/custom.json").unwrap();
                serde_json::to_writer(file, &food).unwrap();
                warp::redirect(Uri::from_static("/"))
            }
        });

    let update_route = warp::path!("update").map({
        move || {
            let update_future = async {
                match update().await {
                    Ok(_) => {
                        println!("[+] Updated!");
                    }
                    Err(e) => {
                        println!("[-] Failed to update: {}", e);
                    }
                };
                // requires rerun to refresh food items, too bad!
            };
            tokio::spawn(update_future);
            warp::redirect(Uri::from_static("/"))
        }
    });

    // unused
    let error = warp::path!("error" / String).map(|error: String| {
        let decoded_error = match decode(&error) {
            Ok(decoded) => decoded.into_owned(),
            Err(_) => error,
        };
        html(
            ErrorHtml {
                error: { match Some(decoded_error) {
                    Some(error) => error,
                    None => "No error message".to_string(),
                }}
            }
            .render_once()
            .unwrap(),
        )
    });
    
    println!("[+] Starting server at http://localhost:8000/...");
    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let routes = static_assets
        .or(index)
        .or(sort)
        .or(search)
        .or(change)
        .or(column)
        .or(show_column_settings)
        .or(paper)
        .or(add)
        .or(remove)
        .or(clear)
        .or(product)
        .or(amount)
        .or(custom)
        .or(insert)
        .or(error)
        .or(update_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
