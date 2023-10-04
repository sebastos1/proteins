pub fn active() -> Vec<String> {
    return vec![
        "kcal".to_string(), 
        "Protein".to_string(), 
        "Karbohydrater".to_string(), 
        "Fett".to_string()
    ];
}

pub fn order<'a>() -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    for (name, unit, _) in big() {
        vec.push((name, unit))
    }
    return vec
}

pub fn trans<'a>() -> Vec<(&'a str, &'a str)> {
    let mut vec = Vec::<(&str, &str)>::new();
    for (new, _, old) in big() {
        vec.push((new, old))
    }
    return vec
}

pub fn big<'a>() -> Vec<(&'a str, &'a str, &'a str)> {
    return vec![
        // new name, unit, original name
        ("kJ", "kJ", "Energi1"),
        ("kcal", "kcal", "Energi2"),
        ("Fett", "g", "Fett"),
        ("Mettet fett", "g", "Mettet"),
        ("Transfett", "g", "Trans"),
        ("Enumettet fett", "g", "Enumet"),
        ("Flerumettet fett", "g", "Flerum"),
        ("Omega-3", "g", "Omega-3"),
        ("Omega-6", "g", "Omega-6"),
        ("Karbohydrater", "g", "Karbo"),
        ("Sukkerarter", "g", "Sukker"),
        ("Stivelse", "g", "Stivel"),
        ("Fiber", "g", "Fiber"),
        ("Protein", "g", "Protein"),
        ("Salt", "g", "NaCl"),
        ("Vann", "g", "Vann"),
        ("Alkohol", "g", "Alko"),
        ("Kolesterol", "mg", "Kolest"),
        ("Vit A", "µg-RE", "Vit A"),
        ("Retinol", "µg", "Retinol"),
        ("Betakaroten", "µg", "B-karo"),
        ("Vit B1", "mg", "Vit B1"),
        ("Vit B2", "mg", "Vit B2"),
        ("Niacin", "mg", "Niacin"),
        ("Vit B6", "mg", "Vit B6"),
        ("Folat", "µg", "Folat"),
        ("Vit B12", "µg", "Vit B12"),
        ("Vit C", "mg", "Vit C"),
        ("Vit D", "µg", "Vit D"),
        ("Vit E", "mg-ATE", "Vit E"),
        ("Kalsium", "mg", "Ca"),
        ("Jern", "mg", "Fe"),
        ("Natrium", "mg", "Na"),
        ("Kalium", "mg", "K"),
        ("Magnesium", "mg", "Mg"),
        ("Sink", "mg", "Zn"),
        ("Selenium", "µg", "Se"),
        ("Kobber", "mg", "Cu"),
        ("Fosfor", "mg", "P"),
        ("Jod", "µg", "I"),
    ];
}