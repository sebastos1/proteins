use http::Uri;
use sailfish::TemplateOnce;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use warp::reply::html;
use warp::Filter;

mod update;
use update::update;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
struct Index<'a> {
    foods: HashMap<String, HashMap<String, String>>,
    x: Vec<&'a str>,
    y: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
struct More<'a> {
    order: Vec<&'a str>,
    product: String,
    foods: HashMap<String, HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let foods: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&std::fs::read_to_string("output.json").unwrap()).unwrap();

    let mut keys: Vec<String> = foods.clone().into_keys().collect();
    keys.sort_by_key(|name| name.to_lowercase()); // alphabetical

    let search = Arc::new(Mutex::new(<Vec<String>>::new()));
    let sort = Arc::new(Mutex::new(<Vec<String>>::new()));

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
    let x = vec!["kJ", "kcal", "Protein", "Karbohydrater", "Fett"];

    let index = warp::path!().map({
        let foods = foods.clone();
        let x = x.clone(); // x
        let keys = keys.clone(); // y
        let search = search.clone();
        let sort = sort.clone();
        move || {
            let foods = foods.clone();
            let keys = keys.clone();
            let x = x.clone();
            let mut search = search.lock().unwrap();
            let mut sort = sort.lock().unwrap();
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
            html(Index { foods, x, y }.render_once().unwrap())
        }
    });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let foods = foods.clone();
            let keys = keys.clone();
            let search = search.clone();
            let sort = sort.clone();
            move |form: Vec<(String, String)>| {
                let foods = foods.clone();
                let keys = keys.clone();
                let mut sort = sort.lock().unwrap();
                let mut search = search.lock().unwrap();

                let mut collected: Vec<(String, u32)> = Vec::new();
                if search.is_empty() {
                    for prod in &keys {
                        if foods[prod].contains_key(&form[0].1) && foods[prod][&form[0].1] != "" {
                            collected.push((
                                prod.clone(),
                                (foods[prod][&form[0].1].trim().parse::<f32>().unwrap() * 100.)
                                    as u32,
                            ));
                        } else {
                            collected.push((prod.clone(), 0. as u32))
                        }
                    }
                } else {
                    for prod in &*search {
                        if foods[prod].contains_key(&form[0].1) && foods[prod][&form[0].1] != "" {
                            collected.push((
                                prod.clone(),
                                (foods[prod][&form[0].1].trim().parse::<f32>().unwrap() * 100.)
                                    as u32,
                            ));
                        } else {
                            collected.push((prod.clone(), 0. as u32))
                        }
                    }
                }
                collected.sort_by(|a, b| b.1.cmp(&a.1));

                let mut sorted: Vec<String> = Vec::new();
                for prod in collected {
                    sorted.push(prod.0)
                }
                *sort = sorted.clone();
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
