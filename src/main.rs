use http::Uri;
use rand::Rng;
use std::thread;
use warp::Filter;
use std::sync::Arc;
use std::sync::Mutex;
use warp::reply::html;
use sailfish::TemplateOnce;
use std::collections::HashMap;
mod update;use update::update;
mod templates;use crate::templates::*;

#[tokio::main]
async fn main() {
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
    let mut keys: Vec<String> = foods.clone().into_keys().collect();
    keys.sort_by_key(|name| name.to_lowercase());

    // vector of valid keys to be displayed, IN ORDER!
    let prods = Arc::new(Mutex::new(<Vec<String>>::new()));

    let sortword = Arc::new(Mutex::new(String::new()));
    let id = Arc::new(Mutex::new(rand::thread_rng().gen::<u32>()));
    let x = vec!["kJ", "kcal", "Protein", "Karbohydrater", "Fett"];
    let order = order();
    let paperitems = Arc::new(Mutex::new(<Vec<(String, f32)>>::new()));
    let ind = Arc::new(Mutex::new(0));
    let entries = 10;

    let index = warp::path!().map({
        let x = x.clone();
        let id = id.clone();
        let ind = ind.clone();
        let keys = keys.clone();
        let prods = prods.clone();
        let foods = foods.clone();
        let entries = entries.clone();
        let sortword = sortword.clone();
        move || {
            let prods = prods.lock().unwrap();

            let y: Vec<String> = if !prods.is_empty() {
                prods.to_vec()
            } else {
                keys.clone()
            };

            html(
                Index {
                    foods: foods.clone(),
                    x: x.clone(),
                    y,
                    rng: id.lock().unwrap().to_string(),
                    sortword: sortword.lock().unwrap().to_string(),
                    ind: *ind.lock().unwrap(),
                    entries: entries.clone(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let ind = ind.clone();
            let keys = keys.clone();
            let prods = prods.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                *ind.lock().unwrap() = 0;

                let mut valid: Vec<String> = Vec::new();
                for item in &keys {
                    if item.to_lowercase().contains(&form[0].1.to_lowercase()) {
                        valid.push(item.to_string())
                    }
                }

                let mut prods = prods.lock().unwrap();
                *prods = valid;
                
                warp::redirect(Uri::from_static("/"))
            }
        });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let ind = ind.clone();
            let keys = keys.clone();
            let foods = foods.clone();
            let prods = prods.clone();
            let sortword = sortword.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                *ind.lock().unwrap() = 0;

                let keys = keys.clone();
                let foods = foods.clone();
                let mut prods = prods.lock().unwrap();
                let mut sortword = sortword.lock().unwrap();
                let mut collected: Vec<(String, u32)> = Vec::new();

                // flex macro
                macro_rules! sort {
                    ($a:expr) => {
                        for prod in &$a {
                            if foods[prod].contains_key(&form[0].1) && foods[prod][&form[0].1] != ""
                            {
                                collected.push((
                                    prod.clone(),
                                    (foods[prod][&form[0].1].trim().parse::<f32>().unwrap() * 100.)
                                        as u32,
                                ));
                            } else {
                                collected.push((prod.clone(), 0. as u32))
                            }
                        }
                    };
                }

                let mut sorted: Vec<String> = Vec::new();
                if prods.is_empty() {
                    sort!(keys)
                } else {
                    sort!(*prods)
                }

                if form[0].1 != sortword.to_string() {
                    collected.sort_by(|a, b| b.1.cmp(&a.1));
                    *sortword = form[0].1.clone();
                } else {
                    collected.sort_by(|a, b| a.1.cmp(&b.1));
                    *sortword = format!("{} (desc.)", form[0].1.clone());
                }

                for item in collected {
                    sorted.push(item.0)
                }
                *prods = sorted.clone();
                
                warp::redirect(Uri::from_static("/"))
            }
        });

    let change = warp::path!("change")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let ind = ind.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                let val = *ind.lock().unwrap();

                if form[0].1 == "up" {
                    *ind.lock().unwrap() = val + entries;
                } else if val >= entries {
                    *ind.lock().unwrap() = val - entries;
                }
        
                warp::redirect(Uri::from_static("/"))
            }
        });

    let product = warp::path("product").and(warp::path::param()).map({
        let order = order.clone();
        let foods = foods.clone();
        move |product: String| {
            html(
                More {
                    order: order.clone(),
                    foods: foods.clone(),
                    product: urlencoding::decode(&product).unwrap().to_string(),
                    multiplier: 1.0,
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let amount = warp::path!("amount")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let order = order.clone();
            let foods = foods.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                html(
                    More {
                        order: order.clone(),
                        foods: foods.clone(),
                        product: form[0].1.to_string(),
                        multiplier: form[1].1.parse::<f32>().unwrap() / 100.,
                    }
                    .render_once()
                    .unwrap(),
                )
            }
        });

    let paper = warp::path("paper").map({
        let id = id.clone();
        let foods = foods.clone();
        let paperitems = paperitems.clone();
        move || {
            html(
                Paper {
                    foods: foods.clone(),
                    paperitems: paperitems.lock().unwrap().to_vec(),
                    rng: id.lock().unwrap().to_string(),
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let add = warp::path!("add")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let paperitems = paperitems.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                let multiplier = form[1].1.parse::<f32>().unwrap();
                let mut paperitems = paperitems.lock().unwrap();

                if paperitems.is_empty() {
                    paperitems.push((form[0].1.clone(), multiplier));
                } else {
                    let mut contains = false;
                    for (i, (prod, mul)) in paperitems.clone().iter().enumerate() {
                        if prod == &form[0].1 {
                            paperitems[i].1 = mul + multiplier;
                            contains = true;
                        }
                    }
                    if contains == false {
                        paperitems.push((form[0].1.clone(), multiplier));
                    }
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    let remove = warp::path!("remove")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let paperitems = paperitems.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                let mut paperitems = paperitems.lock().unwrap();
                paperitems.retain(|item| item.0 != form[0].1);
                warp::redirect(Uri::from_static("/paper"))
            }
        });

    let clear = warp::path!("clear").map({
        let paperitems = paperitems.clone();
        move || {
            *paperitems.lock().unwrap() = <Vec<(String, f32)>>::new();
            warp::redirect(Uri::from_static("/paper"))
        }
    });

    let custom = warp::path!("custom").map(move || html(Custom {}.render_once().unwrap()));

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

                if std::path::Path::new("custom.json").exists()
                    && std::fs::read_to_string("custom.json").unwrap() != ""
                {
                    let mut food: HashMap<String, HashMap<String, String>> =
                        serde_json::from_str(&std::fs::read_to_string("custom.json").unwrap()).unwrap();
                    food.insert(form[0].1.to_string(), nutrients);
                    let file = std::fs::File::create("custom.json").unwrap();
                    serde_json::to_writer(file, &food).unwrap();
                } else {
                    let mut food = HashMap::new();
                    food.insert(form[0].1.to_string(), nutrients);
                    let file = std::fs::File::create("custom.json").unwrap();
                    serde_json::to_writer(file, &food).unwrap();
                }
                warp::redirect(Uri::from_static("/"))
            }
        });

    let updater = warp::path!("updater").map({
        || {
            thread::spawn(|| update()).join().unwrap();
            warp::redirect(Uri::from_static("/"))
        }
    });

    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let routes = index
        .or(static_assets)
        .or(product)
        .or(search)
        .or(sort)
        .or(updater)
        .or(custom)
        .or(insert)
        .or(amount)
        .or(add)
        .or(paper)
        .or(change)
        .or(clear)
        .or(remove);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
