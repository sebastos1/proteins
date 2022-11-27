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
        && &std::fs::read_to_string("output.json").unwrap() != ""
    {
        let custom_foods: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&std::fs::read_to_string("custom.json").unwrap()).unwrap();
        for (k, v) in custom_foods.iter() {
            foods.insert(k.to_string(), v.clone());
        }
    }
    let mut keys: Vec<String> = foods.clone().into_keys().collect();
    keys.sort_by_key(|name| name.to_lowercase());
    let search = Arc::new(Mutex::new(<Vec<String>>::new()));
    let sort = Arc::new(Mutex::new(<Vec<String>>::new()));
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
        let sort = sort.clone();
        let foods = foods.clone();
        let search = search.clone();
        let entries = entries.clone();
        let sortword = sortword.clone();
        move || {
            let search = search.lock().unwrap();
            let mut sort = sort.lock().unwrap();

            let y: Vec<String> = if !sort.is_empty() {
                sort.to_vec()
            } else {
                if !search.is_empty() {
                    search.to_vec()
                } else {
                    keys.clone()
                }
            };
            *sort = Vec::new();
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

    let change = warp::path!("change")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let x = x.clone();
            let id = id.clone();
            let ind = ind.clone();
            let sort = sort.clone();
            let foods = foods.clone();
            let sortword = sortword.clone();
            move |form: Vec<(String, String)>| {
                let lol = *ind.lock().unwrap();
                if form[0].1 == "up" {
                    *ind.lock().unwrap() = lol + entries;
                } else if lol >= entries {
                    *ind.lock().unwrap() = lol - entries;
                }
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();

                html(
                    Index {
                        foods: foods.clone(),
                        x: x.clone(),
                        y: (&*sort.lock().unwrap()).to_vec(),
                        rng: (*id.lock().unwrap()).to_string(),
                        sortword: sortword.lock().unwrap().to_string(),
                        ind: *ind.lock().unwrap(),
                        entries: entries.clone(),
                    }
                    .render_once()
                    .unwrap(),
                )
            }
        });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let id = id.clone();
            let ind = ind.clone();
            let keys = keys.clone();
            let sort = sort.clone();
            let foods = foods.clone();
            let search = search.clone();
            let sortword = sortword.clone();
            move |form: Vec<(String, String)>| {
                *ind.lock().unwrap() = 0;
                let keys = keys.clone();
                let foods = foods.clone();
                let search = search.lock().unwrap();
                let mut sort = sort.lock().unwrap();
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
                if search.is_empty() {
                    sort!(keys)
                } else {
                    sort!(*search)
                }

                if form[0].1 != sortword.to_string() {
                    collected.sort_by(|a, b| b.1.cmp(&a.1));
                    *sortword = form[0].1.clone();
                } else {
                    collected.sort_by(|a, b| a.1.cmp(&b.1));
                    *sortword = format!("{} (desc.)", form[0].1.clone());
                }

                for prod in collected {
                    sorted.push(prod.0)
                }
                *sort = sorted.clone();
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();

                html(
                    Index {
                        foods: foods.clone(),
                        x: x.clone(),
                        y: sorted,
                        rng: id.lock().unwrap().to_string(),
                        sortword: sortword.to_string(),
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
            let search = search.clone();
            move |form: Vec<(String, String)>| {
                *ind.lock().unwrap() = 0;
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

    let updater = warp::path!("updater").map({
        || {
            thread::spawn(|| update()).join().unwrap();
            warp::redirect(Uri::from_static("/"))
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
                nutrients.insert(
                    "kJ".to_string(),
                    format!("{:.1}", form[1].1.parse::<f32>().unwrap() * 4.2),
                );
                nutrients.insert("Source".to_string(), "custom".to_string());

                if !std::path::Path::new("custom.json").exists() {
                    let mut food = HashMap::new();
                    food.insert(&form[0].1, nutrients);
                    let file = std::fs::File::create("custom.json").unwrap();
                    serde_json::to_writer(file, &food).unwrap();
                } else {
                    let content = std::fs::read_to_string("custom.json").unwrap();
                    if content != "" {
                        let mut food: HashMap<String, HashMap<String, String>> =
                            serde_json::from_str(&content).unwrap();
                        food.insert(form[0].1.to_string(), nutrients);
                        let file = std::fs::File::create("custom.json").unwrap();
                        serde_json::to_writer(file, &food).unwrap();
                    } else {
                        let mut food = HashMap::new();
                        food.insert(form[0].1.to_string(), nutrients);
                        let file = std::fs::File::create("custom.json").unwrap();
                        serde_json::to_writer(file, &food).unwrap();
                    }
                }
                warp::redirect(Uri::from_static("/"))
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
            let order = order.clone();
            let foods = foods.clone();
            let paperitems = paperitems.clone();
            move |form: Vec<(String, String)>| {
                *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
                
                let product = &form[0].1;
                let multiplier = form[1].1.parse::<f32>().unwrap();

                let mut paperitems = paperitems.lock().unwrap();
                let iterator = paperitems.clone();

                if paperitems.is_empty() {
                    paperitems.push((product.to_string(), multiplier));
                } else {
                    let mut contains = false;
                    for (i, (prod, mul)) in iterator.iter().enumerate() {
                        if prod == product {
                            paperitems[i].1 = mul + multiplier;
                            contains = true;
                        }
                    }
                    if contains == false {
                        paperitems.push((product.to_string(), multiplier));
                    }
                }
                

                html(
                    More {
                        order: order.clone(),
                        foods: foods.clone(),
                        product: product.to_string(),
                        multiplier,
                    }
                    .render_once()
                    .unwrap(),
                )
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
        .or(change);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
