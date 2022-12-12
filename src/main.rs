use http::Uri;
use rand::Rng;
use std::thread;
use warp::Filter;
use std::sync::Arc;
use std::sync::Mutex;
use warp::reply::html;
use sailfish::TemplateOnce;
use std::collections::HashMap;
mod update;use crate::update::*;
mod templates;use crate::templates::*;

#[tokio::main]
async fn main() {
    let foods = init();
    let mut keys: Vec<String> = foods.clone().into_keys().collect();
        keys.sort_by_key(|name| name.to_lowercase());
    let prods = Arc::new(Mutex::new(<Vec<String>>::new()));
    let word = Arc::new(Mutex::new(String::new()));
    let id = Arc::new(Mutex::new(rand::thread_rng().gen::<u32>()));
    let order = order();
    let paperitems = Arc::new(Mutex::new(<Vec<(String, f32)>>::new()));
    let ind = Arc::new(Mutex::new(0));
    let entries = 10;
    let active = Arc::new(Mutex::new(active()));
    let showcol = Arc::new(Mutex::new(false));

    let index = warp::path!().map({
        let id = id.clone();
        let ind = ind.clone();
        let keys = keys.clone();
        let word = word.clone();
        let order = order.clone();
        let prods = prods.clone();
        let foods = foods.clone();
        let active = active.clone();
        let showcol = showcol.clone();
        let entries = entries.clone();
        move || {
            *id.lock().unwrap() = rand::thread_rng().gen::<u32>();
            let prods = prods.lock().unwrap();
            
            let y: Vec<String> = if !prods.is_empty() {
                prods.to_vec()
            } else {
                keys.clone()
            };

            html(Index {
                y,
                foods: foods.clone(),
                order: order.clone(),
                entries: entries.clone(),
                ind: *ind.lock().unwrap(),
                showcol: *showcol.lock().unwrap(),
                rng: id.lock().unwrap().to_string(),
                word: word.lock().unwrap().to_string(),
                active: active.lock().unwrap().to_vec(),
            }.render_once().unwrap(),)
        }
    });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let ind = ind.clone();
            let keys = keys.clone();
            let word = word.clone();
            let prods = prods.clone();
            move |form: Vec<(String, String)>| {
                *ind.lock().unwrap() = 0;
                *word.lock().unwrap() = String::new();

                let mut valid: Vec<String> = Vec::new();
                let search = form[0].1.split(" ").collect::<Vec<&str>>();
                for item in &keys {
                    if search.iter().all(|term| item.to_lowercase().contains(&term.to_lowercase())) && !valid.contains(item) {
                        valid.push(item.to_string())
                    }
                }
                *prods.lock().unwrap() = valid;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let sort = warp::path!("sort")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let ind = ind.clone();
            let keys = keys.clone();
            let word = word.clone();
            let foods = foods.clone();
            let prods = prods.clone();
            move |form: Vec<(String, String)>| {
                *ind.lock().unwrap() = 0;
                let mut prods = prods.lock().unwrap();
                let mut word = word.lock().unwrap();

                let sort: Vec<String>;
                let mut sorted: Vec<String> = Vec::new();
                if prods.is_empty() {
                    sort = keys.clone();
                } else {
                    sort = (*prods.clone()).to_vec();
                }

                let mut collected: Vec<(String, u32)> = Vec::new();
                for prod in &sort {
                    if foods[prod].contains_key(&form[0].1) && foods[prod][&form[0].1] != "" {
                        collected.push((
                            prod.clone(),
                            (foods[prod][&form[0].1].trim().parse::<f32>().unwrap() * 100.) as u32,
                        ));
                    } else {
                        collected.push((prod.clone(), 0. as u32))
                    }
                }

                if form[0].1 != word.to_string() {
                    collected.sort_by(|a, b| b.1.cmp(&a.1));
                    *word = form[0].1.clone();
                } else {
                    collected.sort_by(|a, b| a.1.cmp(&b.1));
                    *word = format!("{} (desc.)", form[0].1.clone());
                }

                for item in collected {
                    sorted.push(item.0)
                }
                *prods = sorted.clone();

                warp::redirect(Uri::from_static("/"))
            }
        });

    let showcolumn = warp::path!("columns")
        .map({
            let showcol = showcol.clone();
            move || {
                let lol = *showcol.lock().unwrap();
                if lol == false {
                    *showcol.lock().unwrap() = true
                } else {
                    *showcol.lock().unwrap() = false
                }
                warp::redirect(Uri::from_static("/"))
            }
        });
    let column = warp::path!("column")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let active = active.clone();
            move |form: Vec<(String, String)>| {
                let mut new = Vec::<String>::new();
                for (item, _) in &form[1..] {
                    new.push(item.to_string());
                }
                *active.lock().unwrap() = new;
                warp::redirect(Uri::from_static("/"))
            }
        });

    let change = warp::path!("change")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let ind = ind.clone();
            move |form: Vec<(String, String)>| {
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
            html(More {
                multiplier: 1.0,
                order: order.clone(),
                foods: foods.clone(),
                product: urlencoding::decode(&product).unwrap().to_string(),
            }.render_once().unwrap())
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
                html(More {
                    order: order.clone(),
                    foods: foods.clone(),
                    product: form[0].1.to_string(),
                    multiplier: form[1].1.parse::<f32>().unwrap() / 100.,
                }.render_once().unwrap())
            }
        });

    let paper = warp::path("paper").map({
        let id = id.clone();
        let foods = foods.clone();
        let paperitems = paperitems.clone();
        move || {
            html(Paper {
                foods: foods.clone(),
                rng: id.lock().unwrap().to_string(),
                paperitems: paperitems.lock().unwrap().to_vec(),
            }.render_once().unwrap())
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

    let custom = warp::path("custom").map({
        let order = order.clone();
        move || {
            html(Custom {
                order: order.clone()
            }.render_once().unwrap())
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
                if std::path::Path::new("custom.json").exists()
                    && std::fs::read_to_string("custom.json").unwrap() != ""
                {
                    food = serde_json::from_str(&std::fs::read_to_string("custom.json").unwrap())
                        .unwrap();
                } else {
                    food = HashMap::new();
                }
                food.insert(form[0].1.to_string(), nutrients);
                let file = std::fs::File::create("custom.json").unwrap();
                serde_json::to_writer(file, &food).unwrap();
                warp::redirect(Uri::from_static("/"))
            }
        });

    let update = warp::path!("update").map({
        || {
            thread::spawn(|| update()).join().unwrap();
            warp::redirect(Uri::from_static("/"))
        }
    });

    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let routes = static_assets
        .or(index).or(sort).or(search).or(change).or(column).or(showcolumn)
        .or(paper).or(add).or(remove).or(clear)
        .or(product).or(amount)
        .or(custom).or(insert)
        .or(update);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
