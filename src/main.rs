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
mod templates;use crate::templates::{Index, More};

#[tokio::main]
async fn main() {
    let foods: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&std::fs::read_to_string("output.json").unwrap()).unwrap();
    let mut keys: Vec<String> = foods.clone().into_keys().collect();
    keys.sort_by_key(|name| name.to_lowercase());
    let search = Arc::new(Mutex::new(<Vec<String>>::new()));
    let sort = Arc::new(Mutex::new(<Vec<String>>::new()));
    // insanely ugly browser quirk bypass:
    let id = Arc::new(Mutex::new(rand::thread_rng().gen::<u32>()));
    let x = vec!["kJ", "kcal", "Protein", "Karbohydrater", "Fett"];
    let order = vec![
        // "kJ", commented are hard coded in more.html
        // "kcal",
        // "Protein",
        // "Karbohydrater",
        // "Fett",
        "Hvorav mettet",
        "Hvorav enumettet",
        "Hvorav flerumettet",
        "Tilsatt sukker",
        "Vann",
        "Salt",
        "Fiber",
        "Stivelse",
        "Kolesterol",
        "Omega-3",
        "Transfett",
        "Vit A",
        "Vit B1",
        "Vit B2",
        "Vit B6",
        "Vit B12",
        "Vit C",
        "Vit D",
        "Vit E",
        "Alkohol",
    ];

    let index = warp::path!().map({
        let foods = foods.clone();
        let x = x.clone(); // x
        let keys = keys.clone(); // y
        let search = search.clone();
        let sort = sort.clone();
        let id = id.clone();
        move || {
            let foods = foods.clone();
            let x = x.clone();
            let keys = keys.clone();
            let search = search.lock().unwrap();
            let mut sort = sort.lock().unwrap();
            let rng = id.lock().unwrap().to_string();
            let mut y = Vec::new();

            if !sort.is_empty() {
                y = sort.to_vec()
            } else {
                if !search.is_empty() {
                    y = search.to_vec()
                } else {
                    y = keys
                }
            }
            *sort = Vec::new();
            html(Index { foods, x, y, rng }.render_once().unwrap())
        }
    });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let foods = foods.clone();
            let keys = keys.clone();
            let search = search.clone();
            let sort = sort.clone();
            let id = id.clone();
            move |form: Vec<(String, String)>| {
                let foods = foods.clone();
                let keys = keys.clone();
                let search = search.lock().unwrap();
                let mut sort = sort.lock().unwrap();
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

                if search.is_empty() {
                    sort!(keys)
                } else {
                    sort!(*search)
                }

                collected.sort_by(|a, b| b.1.cmp(&a.1));
                let mut sorted: Vec<String> = Vec::new();
                for prod in collected {
                    sorted.push(prod.0)
                }
                *sort = sorted.clone();
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                warp::redirect(Uri::from_static("/"))
            }
        });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let keys = keys.clone();
            let search = search.clone();
            move |form: Vec<(String, String)>| {
                let keys = keys.clone();
                let mut search = search.lock().unwrap();

                let mut sort: Vec<String> = Vec::new();
                for item in keys {
                    if item.to_lowercase().contains(&form[0].1.to_lowercase()) {
                        sort.push(item)
                    }
                }
                *search = sort.clone();
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                warp::redirect(Uri::from_static("/"))
            }
        });

    let product = warp::path("product").and(warp::path::param()).map({
        let order = order.clone();
        let foods = foods.clone();
        move |product: String| {
            let order = order.clone();
            let product = urlencoding::decode(&product).unwrap().to_string();
            let foods = foods.clone();
            html(
                More {
                    order,
                    product,
                    foods,
                }
                .render_once()
                .unwrap(),
            )
        }
    });

    let updater = warp::path!("updater").map({
        || {
            thread::spawn(|| update());
            warp::redirect(Uri::from_static("/"))
        }
    });

    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let routes = index
        .or(static_assets)
        .or(product)
        .or(search)
        .or(updater)
        .or(sort);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
