use http::Uri;
use sailfish::TemplateOnce;
use std::collections::HashMap;
use std::thread;
use warp::reply::html;
use warp::Filter;

mod update;
use update::update;

#[derive(TemplateOnce)]
#[template(path = "index.html")]
struct Index {
    foods: HashMap<String, HashMap<String, String>>,
    sort: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "more.html")]
struct More<'a> {
    order: Vec<&'a str>,
    product: String,
    foods: HashMap<String, HashMap<String, String>>,
}

#[derive(TemplateOnce)]
#[template(path = "header.html")]
struct Header {}

#[tokio::main]
async fn main() {
    let static_assets = warp::path("static").and(warp::fs::dir("static/"));
    let foods: HashMap<String, HashMap<String, String>> =
        serde_json::from_str(&std::fs::read_to_string("output.json").unwrap()).unwrap();
    let mut sort: Vec<String> = foods.clone().into_keys().collect();
    sort.sort_by_key(|name| name.to_lowercase());

    let index = warp::path!().map({
        let foods = foods.clone();
        let sort = sort.clone();
        move || {
            let foods = foods.clone();
            let sort = sort.clone();
            html(Index { foods, sort }.render_once().unwrap())
        }
    });

    let search = warp::path!("search")
        .and(warp::query::<Vec<(String, String)>>())
        .map({
            let foods = foods.clone();
            let sort = sort.clone();
            move |form: Vec<(String, String)>| {
                let foods = foods.clone();
                let alphabetical = sort.clone();
                let mut sort: Vec<String> = Vec::new();
                for item in alphabetical {
                    if item.to_lowercase().contains(&form[0].1.to_lowercase()) {
                        sort.push(item)
                    }
                }
                html(Index { foods, sort }.render_once().unwrap())
            }
        });

    let updater = warp::path!("updater").map({
        move || {
            thread::spawn(|| update());
            warp::redirect(Uri::from_static("/"))
        }
    });

    // auto format moment
    let order = vec![
        // "kJ", "kcal",
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

    let routes = index.or(static_assets).or(product).or(search).or(updater);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
