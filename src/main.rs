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
    let language = Arc::new(Mutex::from(String::from("no")));

    // food data
    let food_data_map = Arc::new(Mutex::new(load_data().await.unwrap()));
    let food_names: Vec<String> = {
        let map_guard = food_data_map.lock().unwrap();
        map_guard.keys().cloned().collect()
    };
    let food_names = Arc::new(Mutex::new(food_names));
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
        let language = language.clone();
        let unique_id = unique_id.clone();
        let dictionary = dictionary.clone();
        let food_names = food_names.clone();
        let food_data_map = food_data_map.clone();
        let active_columns = active_columns.clone();
        let entries_cursor = entries_cursor.clone();
        let entries_per_page = entries_per_page.clone();
        let currently_sorting_by = currently_sorting_by.clone();
        let show_column_settings = show_column_settings.clone();
        let foods_currently_shown = foods_currently_shown.clone();
        move || {
            *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
            let foods_currently_shown = foods_currently_shown.lock().unwrap();

            let foods_to_show: Vec<String> = if !foods_currently_shown.is_empty() {
                foods_currently_shown.to_vec()
            } else {
                food_names.lock().unwrap().clone()
            };

            html(
                Index {
                    foods_to_show,
                    language: language.lock().unwrap().clone(),
                    dictionary: dictionary.clone(),
                    food_data_map: food_data_map.lock().unwrap().clone(),
                    entries_per_page: entries_per_page.clone(),
                    rng: unique_id.lock().unwrap().to_string(),
                    entries_cursor: *entries_cursor.lock().unwrap(),
                    active_columns: active_columns.lock().unwrap().to_vec(),
                    show_column_settings: *show_column_settings.lock().unwrap(),
                    currently_sorting_by: currently_sorting_by.lock().unwrap().to_string(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let food_names = food_names.clone();
            let entries_cursor = entries_cursor.clone();
            let foods_currently_shown = foods_currently_shown.clone();
            move |form: Vec<(String, String)>| {
                *entries_cursor.lock().unwrap() = 0;
                
                let search_terms = form[0].1.split_whitespace().collect::<Vec<&str>>();

                let valid: Vec<String> = food_names.lock().unwrap().iter()
                    .filter(|&item| {
                        search_terms.iter().all(|term| item.to_lowercase().contains(term))
                    })
                    .cloned()
                    .collect();

                *foods_currently_shown.lock().unwrap() = valid;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let food_names = food_names.clone();
            let food_data_map = food_data_map.clone();
            let entries_cursor = entries_cursor.clone();
            let currently_sorting_by = currently_sorting_by.clone();
            let foods_currently_shown = foods_currently_shown.clone();
            move |form: Vec<(String, String)>| {
                let sort_by = &form[0].1;
                *entries_cursor.lock().unwrap() = 0;

                let foods_to_sort = {
                    let foods = foods_currently_shown.lock().unwrap();
                    if foods.is_empty() {
                        food_names.lock().unwrap().clone()
                    } else {
                        foods.clone()
                    }  
                };

                let mut currently_sorting_by = currently_sorting_by.lock().unwrap();
                let descending = sort_by == &*currently_sorting_by;
                if descending {
                    *currently_sorting_by = format!("{sort_by} (asc.)");
                } else {
                    *currently_sorting_by = sort_by.clone();
                }

                let food_data_map = food_data_map.lock().unwrap();
                let mut sorted_tuples: Vec<(String, u32)> = foods_to_sort.iter()
                    .map(|prod| {
                        let binding = "".to_string();
                        let value = food_data_map[prod].get(sort_by).unwrap_or(&binding);
                        let num_value = value.trim().parse::<f32>().unwrap_or(0.0) * 100.0;
                        (prod.clone(), num_value as u32)
                    })
                    .collect();

                sorted_tuples.sort_by(|a, b| {
                    if descending {
                        a.1.cmp(&b.1)                        
                    } else {
                        b.1.cmp(&a.1)
                    }
                });

                let sorted: Vec<String> = sorted_tuples.into_iter().map(|(name, _)| name).collect();
                *foods_currently_shown.lock().unwrap() = sorted;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let show_column_settings = warp::path!("columns").map({
        let show_column_settings = show_column_settings.clone();
        move || {
            if *show_column_settings.lock().unwrap() {
                *show_column_settings.lock().unwrap() = false;
            } else {
                *show_column_settings.lock().unwrap() = true;
            }
            warp::redirect(Uri::from_static("/"))
        }
    });

    let column = warp::path!("column")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let active_columns = active_columns.clone();
            move |form: Vec<(String, String)>| {
                let new_columns: Vec<String> = form.into_iter()
                    .skip(1)
                    .map(|(item, _)| item)
                    .collect();
                *active_columns.lock().unwrap() = new_columns;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let scroll_list = warp::path!("scroll_list")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let entries_cursor = entries_cursor.clone();
            move |form: Vec<(String, String)>| {
                let mut cursor = entries_cursor.lock().unwrap();
                if form[0].1 == "up" {
                    *cursor += entries_per_page;
                } else if *cursor >= entries_per_page {
                    *cursor -= entries_per_page;
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    let product = warp::path("product").and(warp::path::param()).map({
        let language = language.clone();
        let dictionary = dictionary.clone();
        let food_data_map = food_data_map.clone();
        move |product: String| {
            html(
                More {
                    multiplier: 1.0,
                    dictionary: dictionary.clone(),
                    language: language.lock().unwrap().clone(),
                    food_data_map: food_data_map.lock().unwrap().clone(),
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
            let language = language.clone();
            let unique_id = unique_id.clone();
            let dictionary = dictionary.clone();
            let food_data_map = food_data_map.clone();
            move |form: Vec<(String, String)>| {
                *unique_id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                html(
                    More {
                        dictionary: dictionary.clone(),
                        product: form[0].1.to_string(),
                        language: language.lock().unwrap().clone(),
                        food_data_map: food_data_map.lock().unwrap().clone(),
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
                    food_data_map: food_data_map.lock().unwrap().clone(),
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

                if let Some((_, existing_multiplier)) = custom_meal_items.iter_mut().find(|(prod, _)| prod == &form[0].1) {
                    *existing_multiplier += multiplier;
                } else {
                    custom_meal_items.push((form[0].1.clone(), multiplier));
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

    // todo, doesnt work
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
                if std::path::Path::new(CUSTOM_PATH).exists()
                    && std::fs::read_to_string(CUSTOM_PATH).unwrap() != ""
                {
                    food = serde_json::from_str(&std::fs::read_to_string(CUSTOM_PATH).unwrap())
                        .unwrap();
                } else {
                    food = HashMap::new();
                }
                food.insert(form[0].1.to_string(), nutrients);
                let file = std::fs::File::create(CUSTOM_PATH).unwrap();
                serde_json::to_writer(file, &food).unwrap();
                warp::redirect(Uri::from_static("/"))
            }
        });

    let toggle_lang = warp::path!("toggle_lang")
        .map({
            let language = language.clone();
            move || {
                if *language.lock().unwrap() == "no" {
                    *language.lock().unwrap() = "en".to_string();
                } else {
                    *language.lock().unwrap() = "no".to_string();
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    // todo, not implemented for anything
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

    let update_route = warp::path!("update").and_then({
        let food_data_inner = food_data_map.clone();
        move || {           
            let food_data_clone = food_data_inner.clone();
            async move {                
                match update_food_data().await {
                    Ok(new_data) => {                       
                        let mut food_data_map = food_data_clone.lock().unwrap();                        
                        *food_data_map = new_data;
                    }
                    Err(e) => {
                        println!("[-] Failed to update: {}", e);
                    }
                };
                Ok::<_, warp::Rejection>(warp::redirect(Uri::from_static("/")))
            }
        }
    });
    
    println!("[+] Starting server at http://localhost:8000/...");
    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let routes = static_assets
        .or(index)
        .or(sort)
        .or(search)
        .or(scroll_list)
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
        .or(toggle_lang)
        .or(error)
        .or(update_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
